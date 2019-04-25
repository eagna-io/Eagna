use utils::{market, user};

fn main() {
    let conn = librohan::PgConnectionFactory::new_with_env()
        .establish()
        .unwrap();

    let users = vec![user::Alice, user::Bob, user::Rohan];
    users.iter().for_each(|u| {
        u.save(&conn);
    });

    let markets = vec![market::preparing_market()];
    markets.iter().for_each(|m| {
        m.save(&conn);
    });
}
