use crate::{
    Result,
    line_items::model::{EditLineItemForm, LineItem, NewLineItemForm},
    schema::{line_item_dates, line_items},
};
use diesel::prelude::*;

pub(crate) fn all_for_quote<S: AsRef<str>>(
    connection: &mut SqliteConnection,
    quote_id: S,
) -> Result<Vec<LineItem>> {
    let records = line_items::table
        .inner_join(line_item_dates::table)
        .select(LineItem::as_select())
        .filter(line_item_dates::quote_id.eq(&quote_id.as_ref()))
        .get_results(connection)?;

    Ok(records)
}

pub(crate) fn all_for_line_item_date<S: AsRef<str>>(
    connection: &mut SqliteConnection,
    line_item_date_id: S,
) -> Result<Vec<LineItem>> {
    let records = line_items::table
        .filter(line_items::line_item_date_id.eq(&line_item_date_id.as_ref()))
        .get_results(connection)?;
    Ok(records)
}

pub(crate) fn read<S: AsRef<str>>(connection: &mut SqliteConnection, id: S) -> Result<LineItem> {
    let record = line_items::table
        .filter(line_items::id.eq(&id.as_ref()))
        .get_result(connection)?;
    Ok(record)
}

pub(crate) fn insert(
    connection: &mut SqliteConnection,
    form: &NewLineItemForm,
) -> Result<LineItem> {
    let record: LineItem = form.into();

    diesel::dsl::insert_into(line_items::table)
        .values(&record)
        .execute(connection)?;

    Ok(record)
}

pub(crate) fn update(
    connection: &mut SqliteConnection,
    form: &EditLineItemForm,
) -> Result<LineItem> {
    let record: LineItem = form.into();

    diesel::dsl::update(line_items::table)
        .set((
            line_items::name.eq(&record.name),
            line_items::description.eq(&record.description),
            line_items::quantity.eq(&record.quantity),
            line_items::unit_price.eq(&record.unit_price),
            line_items::updated_at.eq(&record.updated_at),
        ))
        .filter(line_items::id.eq(&record.id))
        .execute(connection)?;

    read(connection, &record.id)
}

pub(crate) fn delete<S: AsRef<str>>(connection: &mut SqliteConnection, id: S) -> Result<LineItem> {
    let record = read(connection, &id)?;

    _ = diesel::dsl::delete(line_items::table)
        .filter(line_items::id.eq(&id.as_ref()))
        .execute(connection)?;

    Ok(record)
}

pub(crate) fn delete_all_for_quote<S: AsRef<str>>(
    tx: &mut SqliteConnection,
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

pub(crate) fn delete_all_for_date<S: AsRef<str>>(tx: &mut SqliteConnection, id: S) -> Result {
    _ = diesel::dsl::delete(line_items::table)
        .filter(line_items::line_item_date_id.eq(&id.as_ref()))
        .execute(tx)?;

    Ok(())
}
