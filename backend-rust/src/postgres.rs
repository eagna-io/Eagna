pub mod schema;
pub use self::types::{MarketStatus, OrderType};

use diesel::{pg::PgConnection, result::ConnectionError, Connection};

#[derive(Debug, Clone)]
pub struct ConnectionFactory {
    url: String,
}

impl ConnectionFactory {
    pub fn new_with_env() -> ConnectionFactory {
        let db_url = std::env::var("PG_URL").expect("PG_URL is not presented");
        ConnectionFactory::new(db_url)
    }

    pub fn new(url: String) -> ConnectionFactory {
        ConnectionFactory { url: url }
    }

    pub fn establish(&self) -> Result<PgConnection, ConnectionError> {
        PgConnection::establish(self.url.as_str())
    }
}

pub mod types {
    #[derive(Debug, PartialEq, Eq, DbEnum, Serialize, Deserialize)]
    #[DieselType = "Market_status"]
    pub enum MarketStatus {
        Preparing,
        Open,
        Closed,
        Settled,
    }

    #[derive(Debug, PartialEq, Eq, DbEnum, Serialize, Deserialize)]
    #[DieselType = "Order_type"]
    pub enum OrderType {
        Normal,
        InitialSupply,
        Raward,
        Failure,
    }
}
