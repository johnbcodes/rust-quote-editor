use crate::{
    line_item_dates::model::{LineItemDate, LineItemDateForm},
    line_items,
    time::{DateSql, DateTimeSql},
    Result,
};
use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{Row, Transaction};

pub(crate) async fn all<S: AsRef<str>>(
    pool: &Pool<SqliteConnectionManager>,
    quote_id: S,
) -> Result<Vec<LineItemDate>> {
    // language=SQL
    let sql = r#"
      select
        id,
        quote_id,
        date,
        created_at,
        updated_at
      from line_item_dates
      where quote_id = ?
      order by date
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

pub(crate) async fn read<S: AsRef<str>>(
    pool: &Pool<SqliteConnectionManager>,
    id: S,
) -> Result<LineItemDate> {
    let connection = pool.get()?;
    read_from_connection(&connection, id)
}

fn read_from_connection<S: AsRef<str>>(
    connection: &PooledConnection<SqliteConnectionManager>,
    id: S,
) -> Result<LineItemDate> {
    // language=SQL
    let sql = r#"
      select
        id,
        quote_id,
        date,
        created_at,
        updated_at
      from line_item_dates
      where id = ?
    "#;
    let mut statement = connection.prepare_cached(sql)?;
    let record = statement.query_row([id.as_ref()], map_result)?;
    Ok(record)
}

pub(crate) async fn insert(
    pool: &Pool<SqliteConnectionManager>,
    form: &LineItemDateForm,
) -> Result<LineItemDate> {
    let record: LineItemDate = form.into();
    // language=SQL
    let sql = r#"
        insert into line_item_dates
            (id, quote_id, "date", created_at, updated_at)
        values
            (?, ?, ?, ?, ?);
    "#;
    let connection = pool.get()?;
    let mut statement = connection.prepare_cached(sql)?;
    statement.execute((
        &record.id,
        &record.quote_id,
        &DateSql(record.date),
        &DateTimeSql(record.created_at),
        &DateTimeSql(record.updated_at),
    ))?;

    Ok(record)
}

pub(crate) async fn update(
    pool: &Pool<SqliteConnectionManager>,
    form: &LineItemDateForm,
) -> Result<LineItemDate> {
    let record: LineItemDate = form.into();
    // language=SQL
    let sql = r#"
        update line_item_dates
            set "date" = ?,
                updated_at = ?
        where id = ?;
    "#;
    let connection = pool.get()?;
    let mut statement = connection.prepare_cached(sql)?;
    statement.execute((
        &DateSql(record.date),
        &DateTimeSql(record.updated_at),
        &record.id,
    ))?;
    read_from_connection(&connection, &record.id)
}

pub(crate) async fn delete<S: AsRef<str>>(
    pool: &Pool<SqliteConnectionManager>,
    id: S,
) -> Result<LineItemDate> {
    let record = read(pool, &id).await?;
    let mut connection = pool.get()?;
    let mut tx = connection.transaction()?;
    line_items::query::delete_all_for_date(&mut tx, &id)?;
    delete_line_item_date(&mut tx, &id)?;
    tx.commit()?;
    Ok(record)
}

fn delete_line_item_date<S: AsRef<str>>(tx: &mut Transaction<'_>, id: S) -> Result {
    // language=SQL
    let sql = r#"delete from line_item_dates where id = ?"#;
    let mut statement = tx.prepare_cached(sql)?;
    statement.execute([&id.as_ref()])?;
    Ok(())
}

pub(crate) fn delete_all_for_quote<S: AsRef<str>>(tx: &mut Transaction<'_>, id: S) -> Result {
    // language=SQL
    let sql = r#"
      delete from line_item_dates
      where id in (
        select
          id
        from line_item_dates
        where quote_id = ?
      );
    "#;
    line_items::query::delete_all_for_quote(tx, &id)?;
    let mut statement = tx.prepare_cached(sql)?;
    statement.execute([&id.as_ref()])?;
    Ok(())
}

#[inline]
fn map_result(row: &Row<'_>) -> rusqlite::Result<LineItemDate> {
    Ok(LineItemDate {
        id: row.get(0)?,
        quote_id: row.get(1)?,
        date: row.get::<_, DateSql>(2)?.0,
        created_at: row.get::<_, DateTimeSql>(3)?.0,
        updated_at: row.get::<_, DateTimeSql>(4)?.0,
    })
}
