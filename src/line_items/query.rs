use crate::{
    line_items::model::{LineItem, LineItemForm},
    schema::{line_item_dates, line_items},
    Result,
};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

pub(crate) async fn all_for_quote<S: AsRef<str>>(
    pool: &Pool<ConnectionManager<SqliteConnection>>,
    quote_id: S,
) -> Result<Vec<LineItem>> {
    let mut connection = pool.get()?;
    let records = line_items::table
        .inner_join(line_item_dates::table)
        .select(LineItem::as_select())
        .filter(line_item_dates::quote_id.eq(&quote_id.as_ref()))
        .get_results(&mut connection)?;

    Ok(records)
}

pub(crate) async fn all_for_line_item_date<S: AsRef<str>>(
    pool: &Pool<ConnectionManager<SqliteConnection>>,
    line_item_date_id: S,
) -> Result<Vec<LineItem>> {
    let mut connection = pool.get()?;
    let records = line_items::table
        .filter(line_items::line_item_date_id.eq(&line_item_date_id.as_ref()))
        .get_results(&mut connection)?;
    Ok(records)
}

pub(crate) async fn read<S: AsRef<str>>(
    pool: &Pool<ConnectionManager<SqliteConnection>>,
    id: S,
) -> Result<LineItem> {
    let mut connection = pool.get()?;
    read_from_connection(&mut connection, id)
}

fn read_from_connection<S: AsRef<str>>(
    connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
    id: S,
) -> Result<LineItem> {
    let record = line_items::table
        .filter(line_items::id.eq(&id.as_ref()))
        .get_result(connection)?;
    Ok(record)
}

pub(crate) async fn insert(
    pool: &Pool<ConnectionManager<SqliteConnection>>,
    form: &LineItemForm,
) -> Result<LineItem> {
    let record: LineItem = form.into();

    let mut connection = pool.get()?;
    diesel::dsl::insert_into(line_items::table)
        .values(&record)
        .execute(&mut connection)?;

    Ok(record)
}

pub(crate) async fn update(
    pool: &Pool<ConnectionManager<SqliteConnection>>,
    form: &LineItemForm,
) -> Result<LineItem> {
    let record: LineItem = form.into();

    let mut connection = pool.get()?;
    diesel::dsl::update(line_items::table)
        .set((
            line_items::name.eq(&record.name),
            line_items::description.eq(&record.description),
            line_items::quantity.eq(&record.quantity),
            line_items::unit_price.eq(&record.unit_price),
            line_items::updated_at.eq(&record.updated_at),
        ))
        .filter(line_items::id.eq(&record.id))
        .execute(&mut connection)?;

    read_from_connection(&mut connection, &record.id)
}

pub(crate) async fn delete<S: AsRef<str>>(
    pool: &Pool<ConnectionManager<SqliteConnection>>,
    id: S,
) -> Result<LineItem> {
    let mut connection = pool.get()?;
    let record = read_from_connection(&mut connection, &id)?;

    _ = diesel::dsl::delete(line_items::table)
        .filter(line_items::id.eq(&id.as_ref()))
        .execute(&mut connection)?;

    Ok(record)
}

pub(crate) fn delete_all_for_quote<S: AsRef<str>>(
    tx: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
    quote_id: S,
) -> Result {
    let line_items1 = diesel::alias!(line_items as line_items1);

    _ = diesel::dsl::delete(line_items::table)
        .filter(
            line_items::id.eq_any(
                line_items1
                    .inner_join(line_item_dates::table)
                    .select(line_items1.field(line_items::id))
                    .filter(line_item_dates::quote_id.eq(&quote_id.as_ref())),
            ),
        )
        .execute(tx)?;

    Ok(())
}

pub(crate) fn delete_all_for_date<S: AsRef<str>>(
    tx: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
    id: S,
) -> Result {
    _ = diesel::dsl::delete(line_items::table)
        .filter(line_items::line_item_date_id.eq(&id.as_ref()))
        .execute(tx)?;

    Ok(())
}
