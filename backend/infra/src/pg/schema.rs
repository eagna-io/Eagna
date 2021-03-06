table! {
    use diesel::sql_types::*;
    use crate::pg::types::*;

    /// Representation of the `accounts` table.
    ///
    /// (Automatically generated by Diesel.)
    accounts (id) {
        /// The `id` column of the `accounts` table.
        ///
        /// Its SQL type is `Uuid`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Uuid,
        /// The `name` column of the `accounts` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        name -> Text,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::pg::types::*;

    /// Representation of the `admins` table.
    ///
    /// (Automatically generated by Diesel.)
    admins (id) {
        /// The `id` column of the `admins` table.
        ///
        /// Its SQL type is `Uuid`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Uuid,
        /// The `email` column of the `admins` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        email -> Text,
        /// The `cred` column of the `admins` table.
        ///
        /// Its SQL type is `Bytea`.
        ///
        /// (Automatically generated by Diesel.)
        cred -> Bytea,
        /// The `salt` column of the `admins` table.
        ///
        /// Its SQL type is `Bytea`.
        ///
        /// (Automatically generated by Diesel.)
        salt -> Bytea,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::pg::types::*;

    /// Representation of the `answers` table.
    ///
    /// (Automatically generated by Diesel.)
    answers (id) {
        /// The `id` column of the `answers` table.
        ///
        /// Its SQL type is `Uuid`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Uuid,
        /// The `account_id` column of the `answers` table.
        ///
        /// Its SQL type is `Uuid`.
        ///
        /// (Automatically generated by Diesel.)
        account_id -> Uuid,
        /// The `poll_id` column of the `answers` table.
        ///
        /// Its SQL type is `Uuid`.
        ///
        /// (Automatically generated by Diesel.)
        poll_id -> Uuid,
        /// The `choice_name` column of the `answers` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        choice_name -> Text,
        /// The `created_at` column of the `answers` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Timestamptz,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::pg::types::*;

    /// Representation of the `choices` table.
    ///
    /// (Automatically generated by Diesel.)
    choices (id) {
        /// The `id` column of the `choices` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `poll_id` column of the `choices` table.
        ///
        /// Its SQL type is `Uuid`.
        ///
        /// (Automatically generated by Diesel.)
        poll_id -> Uuid,
        /// The `name` column of the `choices` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        name -> Text,
        /// The `color` column of the `choices` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        color -> Text,
        /// The `idx` column of the `choices` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        idx -> Int4,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::pg::types::*;

    /// Representation of the `comments` table.
    ///
    /// (Automatically generated by Diesel.)
    comments (id) {
        /// The `id` column of the `comments` table.
        ///
        /// Its SQL type is `Uuid`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Uuid,
        /// The `contest_id` column of the `comments` table.
        ///
        /// Its SQL type is `Nullable<Uuid>`.
        ///
        /// (Automatically generated by Diesel.)
        contest_id -> Nullable<Uuid>,
        /// The `account_id` column of the `comments` table.
        ///
        /// Its SQL type is `Uuid`.
        ///
        /// (Automatically generated by Diesel.)
        account_id -> Uuid,
        /// The `answer_id` column of the `comments` table.
        ///
        /// Its SQL type is `Nullable<Uuid>`.
        ///
        /// (Automatically generated by Diesel.)
        answer_id -> Nullable<Uuid>,
        /// The `created_at` column of the `comments` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Timestamptz,
        /// The `content` column of the `comments` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        content -> Text,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::pg::types::*;

    /// Representation of the `contests` table.
    ///
    /// (Automatically generated by Diesel.)
    contests (id) {
        /// The `id` column of the `contests` table.
        ///
        /// Its SQL type is `Uuid`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Uuid,
        /// The `status` column of the `contests` table.
        ///
        /// Its SQL type is `Contest_status`.
        ///
        /// (Automatically generated by Diesel.)
        status -> Contest_status,
        /// The `title` column of the `contests` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        title -> Text,
        /// The `category` column of the `contests` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        category -> Text,
        /// The `event_start_at` column of the `contests` table.
        ///
        /// Its SQL type is `Nullable<Timestamptz>`.
        ///
        /// (Automatically generated by Diesel.)
        event_start_at -> Nullable<Timestamptz>,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::pg::types::*;

    /// Representation of the `polls` table.
    ///
    /// (Automatically generated by Diesel.)
    polls (id) {
        /// The `id` column of the `polls` table.
        ///
        /// Its SQL type is `Uuid`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Uuid,
        /// The `status` column of the `polls` table.
        ///
        /// Its SQL type is `Poll_status`.
        ///
        /// (Automatically generated by Diesel.)
        status -> Poll_status,
        /// The `contest_id` column of the `polls` table.
        ///
        /// Its SQL type is `Uuid`.
        ///
        /// (Automatically generated by Diesel.)
        contest_id -> Uuid,
        /// The `title` column of the `polls` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        title -> Text,
        /// The `duration_sec` column of the `polls` table.
        ///
        /// Its SQL type is `Nullable<Int4>`.
        ///
        /// (Automatically generated by Diesel.)
        duration_sec -> Nullable<Int4>,
        /// The `idx` column of the `polls` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        idx -> Int4,
        /// The `created_at` column of the `polls` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Timestamptz,
        /// The `resolved_at` column of the `polls` table.
        ///
        /// Its SQL type is `Nullable<Timestamptz>`.
        ///
        /// (Automatically generated by Diesel.)
        resolved_at -> Nullable<Timestamptz>,
        /// The `resolved_choice_name` column of the `polls` table.
        ///
        /// Its SQL type is `Nullable<Text>`.
        ///
        /// (Automatically generated by Diesel.)
        resolved_choice_name -> Nullable<Text>,
    }
}

joinable!(answers -> accounts (account_id));
joinable!(answers -> polls (poll_id));
joinable!(choices -> polls (poll_id));
joinable!(comments -> accounts (account_id));
joinable!(comments -> answers (answer_id));
joinable!(comments -> contests (contest_id));
joinable!(polls -> contests (contest_id));

allow_tables_to_appear_in_same_query!(
    accounts,
    admins,
    answers,
    choices,
    comments,
    contests,
    polls,
);
