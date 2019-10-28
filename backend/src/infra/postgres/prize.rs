//! # Develop Design Note
//! インフラ層は他の層（ドメイン層やアプリケーション層）への知識を
//! 全く持たない。
//! よってQueryの結果としてドメイン層のモデルを返すなどはしない。
use super::{schema::prizes, Postgres};
use chrono::{DateTime, Utc};
use diesel::{pg::expression::dsl::any, prelude::*, result::Error as PgError};
use uuid::Uuid;

pub trait PostgresPrizeInfra {
    fn save_prize<'a>(&self, new_prize: NewPrize<'a>) -> Result<(), failure::Error>;

    fn query_all_prizes(&self) -> Result<Vec<QueryPrize>, failure::Error>;

    fn query_available_prizes(&self) -> Result<Vec<QueryPrize>, failure::Error>;

    fn query_prizes(&self, prize_id_list: &[Uuid]) -> Result<Vec<QueryPrize>, failure::Error>;

    fn query_prize(&self, prize_id: &Uuid) -> Result<Option<QueryPrize>, failure::Error>;
}

pub struct NewPrize<'a> {
    pub id: &'a Uuid,
    pub name: &'a str,
    pub description: &'a str,
    pub thumbnail_url: &'a str,
    pub point: u32,
    pub available: bool,
    pub created: &'a DateTime<Utc>,
}

pub struct QueryPrize {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub thumbnail_url: String,
    pub point: u32,
    pub available: bool,
    pub created: DateTime<Utc>,
}

impl PostgresPrizeInfra for Postgres {
    fn save_prize<'a>(&self, new_prize: NewPrize<'a>) -> Result<(), failure::Error> {
        diesel::insert_into(prizes::table)
            .values(InsertablePrize {
                id: new_prize.id,
                name: new_prize.name,
                description: new_prize.description,
                thumbnail_url: new_prize.thumbnail_url,
                point: new_prize.point as i32,
                available: new_prize.available,
            })
            .execute(&self.conn)?;
        Ok(())
    }

    fn query_all_prizes(&self) -> Result<Vec<QueryPrize>, failure::Error> {
        Ok(prizes::table
            .load::<QueryablePrize>(&self.conn)?
            .into_iter()
            .map(QueryablePrize::into)
            .collect())
    }

    fn query_available_prizes(&self) -> Result<Vec<QueryPrize>, failure::Error> {
        Ok(prizes::table
            .filter(prizes::available.eq(true))
            .load::<QueryablePrize>(&self.conn)?
            .into_iter()
            .map(QueryablePrize::into)
            .collect())
    }

    fn query_prizes(&self, prize_id_list: &[Uuid]) -> Result<Vec<QueryPrize>, failure::Error> {
        Ok(prizes::table
            .filter(prizes::id.eq(any(prize_id_list)))
            .load::<QueryablePrize>(&self.conn)?
            .into_iter()
            .map(QueryablePrize::into)
            .collect())
    }

    fn query_prize(&self, prize_id: &Uuid) -> Result<Option<QueryPrize>, failure::Error> {
        match prizes::table
            .filter(prizes::id.eq(prize_id))
            .first::<QueryablePrize>(&self.conn)
        {
            Ok(raw_prize) => Ok(Some(QueryPrize {
                id: raw_prize.id,
                name: raw_prize.name,
                description: raw_prize.description,
                thumbnail_url: raw_prize.thumbnail_url,
                point: raw_prize.point as u32,
                available: raw_prize.available,
                created: raw_prize.created,
            })),
            Err(PgError::NotFound) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }
}

#[derive(Insertable)]
#[table_name = "prizes"]
pub struct InsertablePrize<'a> {
    id: &'a Uuid,
    name: &'a str,
    description: &'a str,
    thumbnail_url: &'a str,
    point: i32,
    available: bool,
}

#[derive(Queryable)]
pub struct QueryablePrize {
    id: Uuid,
    name: String,
    description: String,
    thumbnail_url: String,
    point: i32,
    available: bool,
    created: DateTime<Utc>,
}

impl Into<QueryPrize> for QueryablePrize {
    fn into(self) -> QueryPrize {
        QueryPrize {
            id: self.id,
            name: self.name,
            description: self.description,
            thumbnail_url: self.thumbnail_url,
            point: self.point as u32,
            available: self.available,
            created: self.created,
        }
    }
}
