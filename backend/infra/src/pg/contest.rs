use super::{
    schema::{answers, choices, contests, polls},
    types::{ContestStatus, PollStatus},
    {Connection, Postgres, GLOBAL_PG},
};
use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use diesel::prelude::*;
use domain::{account::*, contest::*};
use std::collections::HashMap;
use uuid::Uuid;

pub struct ContestRepository {
    pg: Postgres,
}

impl ContestRepository {
    pub fn new() -> Self {
        ContestRepository {
            pg: GLOBAL_PG.as_ref().clone(),
        }
    }
}

#[async_trait]
impl domain::contest::ContestRepository for ContestRepository {
    async fn find_by_id(&mut self, id: ContestId) -> anyhow::Result<Option<Contest>> {
        self.pg
            .try_with_conn(move |conn| find_by_id(&conn, id))
            .await
    }

    async fn save(&mut self, contest: Contest) -> anyhow::Result<()> {
        self.pg
            .try_with_conn(move |conn| save(&conn, &contest))
            .await
    }
}

fn find_by_id(conn: &Connection, id: ContestId) -> anyhow::Result<Option<Contest>> {
    let queried_contest = match contests::table
        .left_join(polls::table)
        .filter(contests::id.eq(id.as_ref()))
        .select((
            contests::status,
            contests::title,
            contests::category,
            contests::event_start_at,
            polls::id.nullable(),
            polls::status.nullable(),
            polls::title.nullable(),
            polls::duration_sec.nullable(),
            polls::idx.nullable(),
            polls::resolved_choice_name.nullable(),
        ))
        .order_by(polls::idx.desc())
        .first::<QueriedContest>(conn)
        .optional()?
    {
        Some(c) => c,
        None => return Ok(None),
    };

    let current_poll = if let Some(poll_id) = queried_contest.poll_id.as_ref() {
        let choices = choices::table
            .filter(choices::poll_id.eq(poll_id))
            .select((choices::name, choices::color))
            .order_by(choices::idx.asc())
            .load::<(String, String)>(conn)?
            .into_iter()
            .map(|(name, color)| Choice {
                name: ChoiceName(name),
                color: ChoiceColor(color),
            })
            .collect::<Vec<_>>();

        // Account毎の最後のAnswerを取得する
        let final_answers = answers::table
            .filter(answers::poll_id.eq(poll_id))
            .select((
                answers::id,
                answers::account_id,
                answers::choice_name,
                answers::created_at,
            ))
            .distinct_on(answers::account_id)
            .order_by((answers::account_id, answers::created_at.desc()))
            .load::<(Uuid, Uuid, String, DateTime<Utc>)>(conn)?
            .into_iter()
            .map(|(id, account_id, choice_name, created_at)| {
                let answer = Answer {
                    id: AnswerId(id),
                    poll_id: PollId(*poll_id),
                    account_id: AccountId(account_id),
                    choice_name: ChoiceName(choice_name),
                    created_at,
                };
                (AccountId(account_id), answer)
            })
            .collect::<HashMap<_, _>>();

        Some(Poll {
            id: PollId(*poll_id),
            contest_id: id,
            status: queried_contest.poll_status.unwrap().into(),
            idx: queried_contest.poll_idx.unwrap() as usize,
            title: queried_contest.poll_title.unwrap(),
            duration: Duration::seconds(queried_contest.poll_duration_sec.unwrap() as i64),
            choices,
            resolved_choice_name: queried_contest
                .poll_resolved_choice_name
                .map(|n| ChoiceName(n)),
            final_answers,
        })
    } else {
        None
    };

    Ok(Some(Contest {
        id,
        status: queried_contest.contest_status.into(),
        title: queried_contest.contest_title,
        category: queried_contest.contest_category,
        event_start_at: queried_contest.contest_event_start_at,
        current_poll,
    }))
}

#[derive(Queryable)]
struct QueriedContest {
    contest_status: ContestStatus,
    contest_title: String,
    contest_category: String,
    contest_event_start_at: Option<DateTime<Utc>>,
    poll_id: Option<Uuid>,
    poll_status: Option<PollStatus>,
    poll_title: Option<String>,
    poll_duration_sec: Option<i32>,
    poll_idx: Option<i32>,
    poll_resolved_choice_name: Option<String>,
}

/*
 * ==========
 * Save
 * ==========
 */
fn save(conn: &Connection, contest: &Contest) -> anyhow::Result<()> {
    // Contest
    let new_contest = NewContest {
        id: contest.id.as_ref(),
        status: contest.status.into(),
        title: contest.title.as_str(),
        category: contest.category.as_str(),
        event_start_at: contest.event_start_at.as_ref(),
    };
    diesel::insert_into(contests::table)
        .values(new_contest)
        .on_conflict(contests::id)
        .do_update()
        .set(new_contest)
        .execute(conn)?;

    if let Some(poll) = contest.current_poll.as_ref() {
        // Poll
        let new_poll = NewPoll {
            id: &poll.id.0,
            status: poll.status.into(),
            contest_id: poll.contest_id.as_ref(),
            title: poll.title.as_str(),
            duration_sec: poll.duration.num_seconds() as i32,
            idx: poll.idx as i32,
        };
        diesel::insert_into(polls::table)
            .values(new_poll)
            .on_conflict(polls::id)
            .do_update()
            .set(new_poll)
            .execute(conn)?;

        // Choices
        let new_choices = poll
            .choices
            .iter()
            .enumerate()
            .map(|(i, choice)| NewChoice {
                poll_id: &poll.id.0,
                name: choice.name.0.as_str(),
                color: choice.color.0.as_str(),
                idx: i as i32,
            })
            .collect::<Vec<_>>();
        diesel::insert_into(choices::table)
            .values(new_choices)
            .on_conflict((choices::poll_id, choices::name))
            .do_nothing()
            .execute(conn)?;

        // Answers
        let new_answers = poll
            .final_answers
            .values()
            .map(|answer| NewAnswer {
                id: &answer.id.0,
                account_id: &answer.account_id.0,
                poll_id: &poll.id.0,
                choice_name: answer.choice_name.0.as_str(),
                created_at: &answer.created_at,
            })
            .collect::<Vec<_>>();
        diesel::insert_into(answers::table)
            .values(new_answers)
            .on_conflict(answers::id)
            .do_nothing()
            .execute(conn)?;
    }

    Ok(())
}

#[derive(Clone, Copy, Insertable, AsChangeset)]
#[table_name = "contests"]
struct NewContest<'a> {
    id: &'a Uuid,
    status: ContestStatus,
    title: &'a str,
    category: &'a str,
    event_start_at: Option<&'a DateTime<Utc>>,
}

#[derive(Clone, Copy, Insertable, AsChangeset)]
#[table_name = "polls"]
struct NewPoll<'a> {
    id: &'a Uuid,
    status: PollStatus,
    contest_id: &'a Uuid,
    title: &'a str,
    duration_sec: i32,
    idx: i32,
}

#[derive(Clone, Copy, Insertable, AsChangeset)]
#[table_name = "choices"]
struct NewChoice<'a> {
    poll_id: &'a Uuid,
    name: &'a str,
    color: &'a str,
    idx: i32,
}

#[derive(Clone, Copy, Insertable, AsChangeset)]
#[table_name = "answers"]
struct NewAnswer<'a> {
    id: &'a Uuid,
    account_id: &'a Uuid,
    poll_id: &'a Uuid,
    choice_name: &'a str,
    created_at: &'a DateTime<Utc>,
}
