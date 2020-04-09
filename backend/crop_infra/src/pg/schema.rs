table! {
    use diesel::sql_types::*;

    /// Representation of the `account_choices` table.
    ///
    /// (Automatically generated by Diesel.)
    account_choices (id) {
        /// The `id` column of the `account_choices` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `account_id` column of the `account_choices` table.
        ///
        /// Its SQL type is `Uuid`.
        ///
        /// (Automatically generated by Diesel.)
        account_id -> Uuid,
        /// The `poll_id` column of the `account_choices` table.
        ///
        /// Its SQL type is `Uuid`.
        ///
        /// (Automatically generated by Diesel.)
        poll_id -> Uuid,
        /// The `choice_name` column of the `account_choices` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        choice_name -> Text,
    }
}

table! {
    use diesel::sql_types::*;

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

    /// Representation of the `comments` table.
    ///
    /// (Automatically generated by Diesel.)
    comments (id) {
        /// The `id` column of the `comments` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `poll_id` column of the `comments` table.
        ///
        /// Its SQL type is `Uuid`.
        ///
        /// (Automatically generated by Diesel.)
        poll_id -> Uuid,
        /// The `account_id` column of the `comments` table.
        ///
        /// Its SQL type is `Uuid`.
        ///
        /// (Automatically generated by Diesel.)
        account_id -> Uuid,
        /// The `content` column of the `comments` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        content -> Text,
        /// The `created_at` column of the `comments` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Timestamptz,
    }
}

table! {
    use diesel::sql_types::*;

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
    }
}

table! {
    use diesel::sql_types::*;

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
        /// The `created_at` column of the `polls` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Timestamptz,
        /// The `end_at` column of the `polls` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        end_at -> Timestamptz,
        /// The `resolved_choice_id` column of the `polls` table.
        ///
        /// Its SQL type is `Nullable<Text>`.
        ///
        /// (Automatically generated by Diesel.)
        resolved_choice_id -> Nullable<Text>,
    }
}

joinable!(account_choices -> accounts (account_id));
joinable!(account_choices -> polls (poll_id));
joinable!(choices -> polls (poll_id));
joinable!(comments -> accounts (account_id));
joinable!(comments -> polls (poll_id));
joinable!(polls -> contests (contest_id));

allow_tables_to_appear_in_same_query!(
    account_choices,
    accounts,
    choices,
    comments,
    contests,
    polls,
);
