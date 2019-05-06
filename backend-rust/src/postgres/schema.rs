table! {
    use diesel::sql_types::*;
    use crate::postgres::types::*;

    markets (id) {
        id -> Int4,
        title -> Text,
        organizer -> Text,
        short_desc -> Text,
        description -> Text,
        lmsr_b -> Int4,
        open_time -> Timestamptz,
        close_time -> Timestamptz,
        status -> Market_status,
        settle_token_id -> Nullable<Int4>,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::postgres::types::*;

    market_tokens (id) {
        id -> Int4,
        name -> Text,
        description -> Text,
        market_id -> Int4,
        initial_price -> Int4,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::postgres::types::*;

    orders (id) {
        id -> Int4,
        user_id -> Int4,
        market_id -> Int4,
        token_id -> Nullable<Int4>,
        in_market_id -> Int4,
        amount_token -> Int4,
        amount_coin -> Int4,
        #[sql_name = "type"]
        type_ -> Order_type,
        time -> Timestamptz,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::postgres::types::*;

    users (id) {
        id -> Int4,
        name -> Text,
        email -> Text,
        hashed_pass -> Text,
    }
}

joinable!(market_tokens -> markets (market_id));
joinable!(orders -> market_tokens (token_id));
joinable!(orders -> markets (market_id));
joinable!(orders -> users (user_id));

allow_tables_to_appear_in_same_query!(
    markets,
    market_tokens,
    orders,
    users,
);
