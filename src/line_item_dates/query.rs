use crate::{
    line_item_dates::model::{LineItemDate, LineItemDateForm},
    line_items,
    schema::line_item_dates,
    Result,
};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

pub(crate) async fn all<S: AsRef<str>>(
    pool: &Pool<ConnectionManager<SqliteConnection>>,
    id: S,
) -> Result<Vec<LineItemDate>> {
    let mut connection = pool.get()?;
    let records = line_item_dates::table
        .filter(line_item_dates::quote_id.eq(&id.as_ref()))
        .get_results(&mut connection)?;
    Ok(records)
}

pub(crate) async fn read<S: AsRef<str>>(
    pool: &Pool<ConnectionManager<SqliteConnection>>,
    id: S,
) -> Result<LineItemDate> {
    let mut connection = pool.get()?;
    read_from_connection(&mut connection, id)
}

fn read_from_connection<S: AsRef<str>>(
    connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
    id: S,
) -> Result<LineItemDate> {
    let record = line_item_dates::table
        .filter(line_item_dates::id.eq(&id.as_ref()))
        .get_result(connection)?;
    Ok(record)
}

pub(crate) async fn insert(
    pool: &Pool<ConnectionManager<SqliteConnection>>,
    form: &LineItemDateForm,
) -> Result<LineItemDate> {
    let record: LineItemDate = form.into();

    let mut connection = pool.get()?;
    diesel::dsl::insert_into(line_item_dates::table)
        .values(&record)
        .execute(&mut connection)?;

    Ok(record)
}

pub(crate) async fn update(
    pool: &Pool<ConnectionManager<SqliteConnection>>,
    form: &LineItemDateForm,
) -> Result<LineItemDate> {
    let record: LineItemDate = form.into();

    let mut connection = pool.get()?;
    diesel::dsl::update(line_item_dates::table)
        .set((
            line_item_dates::date.eq(&record.date),
            line_item_dates::updated_at.eq(&record.updated_at),
        ))
        .filter(line_item_dates::id.eq(&record.id))
        .execute(&mut connection)?;

    read_from_connection(&mut connection, &record.id)
}

pub(crate) async fn delete<S: AsRef<str>>(
    pool: &Pool<ConnectionManager<SqliteConnection>>,
    id: S,
) -> Result<LineItemDate> {
    let mut connection = pool.get()?;

    let record = read_from_connection(&mut connection, &id)?;
    _ = connection.transaction::<_, _, _>(|tx| {
        line_items::query::delete_all_for_date(tx, &id)?;

        _ = diesel::dsl::delete(line_item_dates::table)
            .filter(line_item_dates::id.eq(&id.as_ref()))
            .execute(tx)?;

        Ok::<(), crate::error::AppError>(())
    });

    Ok(record)
}

pub(crate) fn delete_all_for_quote<S: AsRef<str>>(
    tx: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
    id: S,
) -> Result {
    line_items::query::delete_all_for_quote(tx, &id)?;

    _ = diesel::dsl::delete(line_item_dates::table)
        .filter(line_item_dates::quote_id.eq(&id.as_ref()))
        .execute(tx)?;

    Ok(())
}
