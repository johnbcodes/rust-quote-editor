// @generated automatically by Diesel CLI.

diesel::table! {
    line_item_dates (id) {
        id -> Text,
        quote_id -> Text,
        date -> Date,
        created_at -> TimestamptzSqlite,
        updated_at -> TimestamptzSqlite,
    }
}

diesel::table! {
    use currency_rs::diesel2::sqlite::sql_types::Currency;
    use diesel::sql_types::*;

    line_items (id) {
        id -> Text,
        line_item_date_id -> Text,
        name -> Text,
        description -> Nullable<Text>,
        quantity -> Integer,
        unit_price -> Currency,
        created_at -> TimestamptzSqlite,
        updated_at -> TimestamptzSqlite,
    }
}

diesel::table! {
    quotes (id) {
        id -> Text,
        name -> Text,
        created_at -> TimestamptzSqlite,
        updated_at -> TimestamptzSqlite,
    }
}

diesel::joinable!(line_item_dates -> quotes (quote_id));
diesel::joinable!(line_items -> line_item_dates (line_item_date_id));

diesel::allow_tables_to_appear_in_same_query!(line_item_dates, line_items, quotes);
