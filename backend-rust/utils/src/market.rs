use chrono::{DateTime, Duration, Utc};
use librohan::domain::{
    models::{
        lmsr,
        market::{
            Market, MarketDesc, MarketId, MarketOrganizer, MarketShortDesc, MarketTitle, TokenDesc,
            TokenName,
        },
    },
    services::{
        market_store::{NewMarket, NewToken},
        {MarketStore, UserStore},
    },
};
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

static SERIAL_ID: AtomicUsize = AtomicUsize::new(1);

pub fn insert_new_market<S>(store: &mut S, open_time: DateTime<Utc>) -> MarketId
where
    S: MarketStore,
{
    let id = SERIAL_ID.fetch_add(1, Ordering::SeqCst);

    let new_market = NewMarket {
        title: MarketTitle(Arc::new(format!("Market {}", id))),
        organizer: MarketOrganizer(Arc::new(format!("Rohan market.inc"))),
        short_desc: MarketShortDesc(Arc::new(format!("The #{} market", id))),
        description: MarketDesc(Arc::new(format!(
            "Answer to the Ultimate Question of Life, the Universe, and Everything"
        ))),
        lmsr_b: lmsr::B(100),
        open_time: open_time,
        close_time: open_time + Duration::minutes(10),
        tokens: vec![
            NewToken {
                name: TokenName(Arc::new(format!("Alice"))),
                description: TokenDesc(Arc::new(format!("Alice wins"))),
            },
            NewToken {
                name: TokenName(Arc::new(format!("Bob"))),
                description: TokenDesc(Arc::new(format!("Bob wins"))),
            },
        ],
    };

    let market_id = store.insert_market(new_market).unwrap();

    market_id
}

pub fn open_preparing_market<S>(store: &mut S, market_id: &MarketId)
where
    S: MarketStore + UserStore,
{
    let users = store.query_all_user_ids().unwrap();

    let mut locked_store = store.lock_market(market_id).unwrap();
    match locked_store.query_market(market_id).unwrap().unwrap() {
        Market::Preparing(m) => {
            let open_market = m.open_uncheck(&users);
            locked_store
                .update_market_status_to_open(&open_market)
                .unwrap();
        }
        _ => panic!(format!("Market {} is not preparing", market_id.0)),
    }
}
