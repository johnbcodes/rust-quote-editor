use crate::{
    currency::CurrencySql,
    line_item_dates,
    quotes::model::{Quote, QuoteForm},
    time::DateTimeSql,
    Result,
};
use currency_rs::Currency;
use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::Row;

pub(crate) async fn all(pool: &Pool<SqliteConnectionManager>) -> Result<Vec<Quote>> {
    // language=SQL
    let sql = r#"
      select
        id,
        name,
        created_at,
        updated_at
      from quotes
      order by rowid desc
    "#;
    let connection = pool.get()?;
    let mut statement = connection.prepare_cached(sql)?;
    let records = statement
        .query_map([], map_all)
        .unwrap()
        .map(|result| result.unwrap())
        .collect();
    Ok(records)
}

pub(crate) async fn read<S: AsRef<str>>(
    pool: &Pool<SqliteConnectionManager>,
    id: S,
) -> Result<Quote> {
    let connection = pool.get()?;
    read_from_connection(&connection, id)
}

fn read_from_connection<S: AsRef<str>>(
    connection: &PooledConnection<SqliteConnectionManager>,
    id: S,
) -> Result<Quote> {
    // language=SQL
    let sql = r#"
      select
        q.id,
        q.name,
        (select
            coalesce(sum(quantity * li.unit_price), 0)
          from line_items li
            inner join line_item_dates lid on li.line_item_date_id = lid.id
          where lid.quote_id = q.id) as total,
        q.created_at,
        q.updated_at
      from quotes q
      where q.id = ?
    "#;
    let mut statement = connection.prepare_cached(sql)?;
    let record = statement.query_row([id.as_ref()], map_read)?;
    Ok(record)
}

pub(crate) async fn from_line_item_date_id<S: AsRef<str>>(
    pool: &Pool<SqliteConnectionManager>,
    id: S,
) -> Result<Quote> {
    // language=SQL
    let sql = r#"
      select
        q.id,
        q.name,
        (select
            coalesce(sum(quantity * li.unit_price), 0)
          from line_items li
            inner join line_item_dates lid2 on li.line_item_date_id = lid2.id
          where lid2.quote_id = q.id) as total,
        q.created_at,
        q.updated_at
      from line_item_dates lid
        inner join quotes q on lid.quote_id = q.id
      where lid.id = ?
    "#;
    let connection = pool.get()?;
    let mut statement = connection.prepare_cached(sql)?;
    let record = statement.query_row([id.as_ref()], map_read)?;
    Ok(record)
}

pub(crate) async fn insert(
    pool: &Pool<SqliteConnectionManager>,
    form: &QuoteForm,
) -> Result<Quote> {
    let record: Quote = form.into();

    // language=SQL
    let sql = r#"
        insert into quotes
            (id, name, created_at, updated_at)
        values
            (?, ?, ?, ?);
    "#;
    let connection = pool.get()?;
    let mut statement = connection.prepare_cached(sql)?;
    statement.execute((
        &record.id,
        &record.name,
        &DateTimeSql(record.created_at),
        &DateTimeSql(record.updated_at),
    ))?;

    Ok(record)
}

pub(crate) async fn update(
    pool: &Pool<SqliteConnectionManager>,
    form: &QuoteForm,
) -> Result<Quote> {
    let record: Quote = form.into();
    // language=SQL
    let sql = r#"
        update quotes
            set name = ?,
                updated_at = ?
        where id = ?;
    "#;
    let connection = pool.get()?;
    let mut statement = connection.prepare_cached(sql)?;
    statement.execute((&record.name, &DateTimeSql(record.updated_at), &record.id))?;
    read_from_connection(&connection, &record.id)
}

pub(crate) async fn delete<S: AsRef<str>>(
    pool: &Pool<SqliteConnectionManager>,
    id: S,
) -> Result<Quote> {
    let record = read(pool, &id).await?;
    // language=SQL
    let sql = r#"delete from quotes where id = ?"#;
    let mut connection = pool.get()?;
    let mut tx = connection.transaction()?;
    line_item_dates::query::delete_all_for_quote(&mut tx, &id)?;
    {
        let mut statement = tx.prepare_cached(sql)?;
        statement.execute([&id.as_ref()])?;
    }
    tx.commit()?;
    Ok(record)
}

#[inline]
fn map_all(row: &Row<'_>) -> rusqlite::Result<Quote> {
    Ok(Quote {
        id: row.get(0)?,
        name: row.get(1)?,
        total: Currency::new_float(0f64, None),
        created_at: row.get::<_, DateTimeSql>(2)?.0,
        updated_at: row.get::<_, DateTimeSql>(3)?.0,
    })
}

#[inline]
fn map_read(row: &Row<'_>) -> rusqlite::Result<Quote> {
    Ok(Quote {
        id: row.get(0)?,
        name: row.get(1)?,
        total: row.get::<_, CurrencySql>(2)?.0,
        created_at: row.get::<_, DateTimeSql>(3)?.0,
        updated_at: row.get::<_, DateTimeSql>(4)?.0,
    })
}
