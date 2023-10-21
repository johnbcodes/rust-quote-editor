use once_cell::sync::Lazy;
use rusqlite_migration::{Migrations, M};

pub(crate) static MIGRATIONS: Lazy<Migrations<'static>> = Lazy::new(|| {
    Migrations::new(vec![
        M::up(include_str!("../migrations/20230408081854_quotes.up.sql"))
            .down(include_str!("../migrations/20230408081854_quotes.down.sql")),
        M::up(include_str!(
            "../migrations/20230413082629_line_item_dates.up.sql"
        ))
        .down(include_str!(
            "../migrations/20230413082629_line_item_dates.down.sql"
        )),
        M::up(include_str!(
            "../migrations/20230413082726_line_items.up.sql"
        ))
        .down(include_str!(
            "../migrations/20230413082726_line_items.down.sql"
        )),
    ])
});
