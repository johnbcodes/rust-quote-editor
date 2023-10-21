use crate::{
    currency::CurrencySql,
    line_items::model::{LineItem, LineItemForm},
    time::DateTimeSql,
    Result,
};
use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{Row, Transaction};
use tracing::info;

pub(crate) async fn all_for_quote<S: AsRef<str>>(
    pool: &Pool<SqliteConnectionManager>,
    quote_id: S,
) -> Result<Vec<LineItem>> {
    // language=SQL
    let sql = r#"
      select
        li.id,
        li.line_item_date_id,
        li.name,
        li.description,
        li.quantity,
        li.unit_price,
        li.created_at,
        li.updated_at
      from line_items li
        inner join line_item_dates lid on lid.id = li.line_item_date_id
      where lid.quote_id = ?
      order by li.rowid
    "#;
    let connection = pool.get()?;
    let mut statement = connection.prepare_cached(sql)?;
    let records = statement
        .query_map([&quote_id.as_ref()], map_result)
        .unwrap()
        .map(|result| result.unwrap())
        .collect();
    Ok(records)
}

pub(crate) async fn all_for_line_item_date<S: AsRef<str>>(
    pool: &Pool<SqliteConnectionManager>,
    line_item_date_id: S,
) -> Result<Vec<LineItem>> {
    // language=SQL
    let sql = r#"
      select
        id,
        line_item_date_id,
        name,
        description,
        quantity,
        unit_price,
        created_at,
        updated_at
      from line_items
      where line_item_date_id = ?
      order by rowid
    "#;
    let connection = pool.get()?;
    let mut statement = connection.prepare_cached(sql)?;
    let records = statement
        .query_map([&line_item_date_id.as_ref()], map_result)
        .unwrap()
        .map(|result| result.unwrap())
        .collect();
    Ok(records)
}

pub(crate) async fn read<S: AsRef<str>>(
    pool: &Pool<SqliteConnectionManager>,
    id: S,
) -> Result<LineItem> {
    let connection = pool.get()?;
    read_from_connection(&connection, id)
}

fn read_from_connection<S: AsRef<str>>(
    connection: &PooledConnection<SqliteConnectionManager>,
    id: S,
) -> Result<LineItem> {
    // language=SQL
    let sql = r#"
      select
        id,
        line_item_date_id,
        name,
        description,
        quantity,
        unit_price,
        created_at,
        updated_at
      from line_items
      where id = ?
    "#;
    let mut statement = connection.prepare_cached(sql)?;
    let record = statement.query_row([id.as_ref()], map_result)?;
    Ok(record)
}

pub(crate) async fn insert(
    pool: &Pool<SqliteConnectionManager>,
    form: &LineItemForm,
) -> Result<LineItem> {
    let record: LineItem = form.into();

    // language=SQL
    let sql = r#"
        insert into line_items
            (id, line_item_date_id, name, description, quantity, unit_price, created_at, updated_at)
        values
            (?, ?, ?, ?, ?, ?, ?, ?);
    "#;
    let connection = pool.get()?;
    let mut statement = connection.prepare_cached(sql)?;
    statement.execute((
        &record.id,
        &record.line_item_date_id,
        &record.name,
        &record.description,
        &record.quantity,
        &CurrencySql(record.unit_price.clone()),
        &DateTimeSql(record.created_at),
        &DateTimeSql(record.updated_at),
    ))?;

    Ok(record)
}

pub(crate) async fn update(
    pool: &Pool<SqliteConnectionManager>,
    form: &LineItemForm,
) -> Result<LineItem> {
    let record: LineItem = form.into();
    info!("LineItem:\n{:?}", &record);

    // language=SQL
    let sql = r#"
        update line_items
            set name = ?,
                description = ?,
                quantity = ?,
                unit_price = ?,
                updated_at = ?
        where id = ?;
    "#;
    let connection = pool.get()?;
    let mut statement = connection.prepare_cached(sql)?;
    statement.execute((
        &record.name,
        &record.description,
        &record.quantity,
        &CurrencySql(record.unit_price),
        &DateTimeSql(record.updated_at),
        &record.id,
    ))?;
    read_from_connection(&connection, &record.id)
}

pub(crate) async fn delete<S: AsRef<str>>(
    pool: &Pool<SqliteConnectionManager>,
    id: S,
) -> Result<LineItem> {
    let record = read(pool, &id).await?;
    // language=SQL
    let sql = r#"delete from line_items where id = ?"#;
    let connection = pool.get()?;
    let mut statement = connection.prepare_cached(sql)?;
    statement.execute([&id.as_ref()])?;
    Ok(record)
}

pub(crate) fn delete_all_for_quote<S: AsRef<str>>(tx: &mut Transaction<'_>, id: S) -> Result {
    // language=SQL
    let sql = r#"
        delete from line_items
        where id in (
          select
            li.id
          from line_items li
            inner join line_item_dates lid on lid.id = li.line_item_date_id
          where lid.quote_id = ?
        );"#;
    let mut statement = tx.prepare_cached(sql)?;
    statement.execute([&id.as_ref()])?;
    Ok(())
}

pub(crate) fn delete_all_for_date<S: AsRef<str>>(tx: &mut Transaction<'_>, id: S) -> Result {
    // language=SQL
    let sql = r#"
        delete from line_items
        where id in (
          select
            id
          from line_items
          where line_item_date_id = ?
        );"#;
    let mut statement = tx.prepare_cached(sql)?;
    statement.execute([&id.as_ref()])?;
    Ok(())
}

#[inline]
fn map_result(row: &Row<'_>) -> rusqlite::Result<LineItem> {
    Ok(LineItem {
        id: row.get(0)?,
        line_item_date_id: row.get(1)?,
        name: row.get(2)?,
        description: row.get(3)?,
        quantity: row.get(4)?,
        unit_price: row.get::<_, CurrencySql>(5)?.0,
        created_at: row.get::<_, DateTimeSql>(6)?.0,
        updated_at: row.get::<_, DateTimeSql>(7)?.0,
    })
}
