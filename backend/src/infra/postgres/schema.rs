table! {
    use diesel::sql_types::*;
    use crate::infra::postgres::types::*;

    markets (id) {
        id -> Uuid,
        title -> Text,
        description -> Text,
        lmsr_b -> Int4,
        open -> Timestamptz,
        close -> Timestamptz,
        status -> Market_status,
        resolved_token_name -> Nullable<Text>,
        resolved_at -> Nullable<Timestamptz>,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::infra::postgres::types::*;

    market_tokens (unused_id) {
        unused_id -> Int4,
        name -> Text,
        description -> Text,
        thumbnail_url -> Text,
        market_id -> Uuid,
        idx -> Int4,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::infra::postgres::types::*;

    orders (id) {
        id -> Uuid,
        user_id -> Uuid,
        token_name -> Text,
        amount_token -> Int4,
        amount_coin -> Int4,
        time -> Timestamptz,
        market_id -> Uuid,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::infra::postgres::types::*;

    users (id) {
        id -> Uuid,
        name -> Text,
        email -> Text,
        coin -> Int4,
        point -> Int4,
        is_admin -> Bool,
        created -> Timestamptz,
        credential -> Bytea,
        salt -> Bytea,
    }
}

joinable!(market_tokens -> markets (market_id));
joinable!(orders -> markets (market_id));
joinable!(orders -> users (user_id));

allow_tables_to_appear_in_same_query!(
    markets,
    market_tokens,
    orders,
    users,
);
