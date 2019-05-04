use chrono::{DateTime, Duration, Utc};
use diesel::prelude::*;
use librohan::postgres::{schema::markets, MarketStatus};

#[derive(Debug, PartialEq, Eq, Insertable)]
#[table_name = "markets"]
pub struct NewMarket {
    pub title: String,
    pub organizer: String,
    pub short_desc: String,
    pub description: String,
    pub open_time: DateTime<Utc>,
    pub close_time: DateTime<Utc>,
    pub status: MarketStatus,
    pub settle_token_id: Option<i32>,
}

impl NewMarket {
    pub fn save(&self, conn: &PgConnection) -> i32 {
        use librohan::postgres::schema::markets::{columns as market, table as markets};

        diesel::insert_into(markets)
            .values(self)
            .returning(market::id)
            .get_result(conn)
            .expect("Failed to insert market")
    }

    pub fn get_id(&self, conn: &PgConnection) -> i32 {
        use librohan::postgres::schema::markets::{columns as market, table as markets};

        markets
            .select(market::id)
            .filter(market::title.eq(self.title.as_str()))
            .first(conn)
            .expect("Failed to query market")
    }
}

pub fn preparing_market() -> NewMarket {
    NewMarket {
        title: "The preparing market".into(),
        organizer: "Rohan market.inc".into(),
        short_desc: "It will always start 10min after it is created".into(),
        description: "Answer to the Ultimate Question of Life, the Universe, and Everything".into(),
        open_time: Utc::now() - Duration::minutes(10),
        close_time: Utc::now() + Duration::hours(1),
        status: MarketStatus::Preparing,
        settle_token_id: None,
    }
}
