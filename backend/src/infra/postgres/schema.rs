table! {
    use diesel::sql_types::*;
    use crate::infra::postgres::types::*;

    market_prizes (unused_id) {
        unused_id -> Int4,
        market_local_id -> Int4,
        name -> Text,
        thumbnail_url -> Text,
        target -> Text,
        market_id -> Uuid,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::infra::postgres::types::*;

    markets (id) {
        id -> Uuid,
        title -> Text,
        organizer_id -> Uuid,
        description -> Text,
        lmsr_b -> Int4,
        open -> Timestamptz,
        close -> Timestamptz,
        status -> Market_status,
        resolved_token_name -> Nullable<Text>,
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

    orders (unused) {
        unused -> Int4,
        market_local_id -> Int4,
        user_id -> Text,
        token_name -> Nullable<Text>,
        amount_token -> Int4,
        amount_coin -> Int4,
        #[sql_name = "type"]
        type_ -> Order_type,
        time -> Timestamptz,
        market_id -> Uuid,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::infra::postgres::types::*;

    organizers (id) {
        id -> Uuid,
        name -> Text,
        thumbnail_url -> Text,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::infra::postgres::types::*;

    users (fb_uid) {
        fb_uid -> Text,
        name -> Text,
        email -> Text,
        is_admin -> Bool,
        created -> Timestamptz,
    }
}

joinable!(market_prizes -> markets (market_id));
joinable!(market_tokens -> markets (market_id));
joinable!(markets -> organizers (organizer_id));
joinable!(orders -> markets (market_id));
joinable!(orders -> users (user_id));

allow_tables_to_appear_in_same_query!(
    market_prizes,
    markets,
    market_tokens,
    orders,
    organizers,
    users,
);
