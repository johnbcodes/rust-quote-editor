use crate::{
    line_item_dates,
    quotes::model::{Quote, QuoteForm, QuoteWithTotal},
    schema::quotes,
    Result,
};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

pub(crate) async fn all(pool: &Pool<ConnectionManager<SqliteConnection>>) -> Result<Vec<Quote>> {
    let mut connection = pool.get()?;
    let records = quotes::table
        .order_by(quotes::id)
        .get_results(&mut connection)?;
    Ok(records)
}

pub(crate) async fn read<S: AsRef<str>>(
    pool: &Pool<ConnectionManager<SqliteConnection>>,
    id: S,
) -> Result<QuoteWithTotal> {
    let mut connection = pool.get()?;
    read_from_connection(&mut connection, id)
}

fn read_from_connection<S: AsRef<str>>(
    connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
    id: S,
) -> Result<QuoteWithTotal> {
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
    let record = diesel::dsl::sql_query(sql)
        .bind::<diesel::sql_types::Text, _>(id.as_ref())
        .get_result(connection)?;
    Ok(record)
}

pub(crate) async fn from_line_item_date_id<S: AsRef<str>>(
    pool: &Pool<ConnectionManager<SqliteConnection>>,
    id: S,
) -> Result<QuoteWithTotal> {
    // language=SQL
    let sql = r#"
      select
        q.id,
        q.name,
        (select
            coalesce(sum(li.quantity * li.unit_price), 0)
          from line_items li
            inner join line_item_dates lid2 on li.line_item_date_id = lid2.id
          where lid2.quote_id = q.id) as total,
        q.created_at,
        q.updated_at
      from line_item_dates lid
        inner join quotes q on lid.quote_id = q.id
      where lid.id = ?
    "#;
    let mut connection = pool.get()?;
    let record = diesel::dsl::sql_query(sql)
        .bind::<diesel::sql_types::Text, _>(id.as_ref())
        .get_result(&mut connection)?;
    Ok(record)
}

pub(crate) async fn insert(
    pool: &Pool<ConnectionManager<SqliteConnection>>,
    form: &QuoteForm,
) -> Result<Quote> {
    let record: Quote = form.into();

    let mut connection = pool.get()?;
    diesel::dsl::insert_into(quotes::table)
        .values(&record)
        .execute(&mut connection)?;

    Ok(record)
}

pub(crate) async fn update(
    pool: &Pool<ConnectionManager<SqliteConnection>>,
    form: &QuoteForm,
) -> Result<QuoteWithTotal> {
    let record: Quote = form.into();

    let mut connection = pool.get()?;
    diesel::dsl::update(quotes::table)
        .set((
            quotes::name.eq(&record.name),
            quotes::updated_at.eq(&record.updated_at),
        ))
        .filter(quotes::id.eq(&record.id))
        .execute(&mut connection)?;

    read_from_connection(&mut connection, &record.id)
}

pub(crate) async fn delete<S: AsRef<str>>(
    pool: &Pool<ConnectionManager<SqliteConnection>>,
    id: S,
) -> Result<QuoteWithTotal> {
    let mut connection = pool.get()?;
    let record = read_from_connection(&mut connection, &id)?;

    _ = connection.transaction::<_, _, _>(|tx| {
        line_item_dates::query::delete_all_for_quote(tx, &id)?;

        _ = diesel::dsl::delete(quotes::table)
            .filter(quotes::id.eq(&id.as_ref()))
            .execute(tx)?;

        Ok::<(), crate::error::AppError>(())
    });
    Ok(record)
}
