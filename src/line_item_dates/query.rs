use crate::{
    line_item_dates::model::{EditLineItemDateForm, LineItemDate, NewLineItemDateForm},
    line_items,
    schema::line_item_dates,
    Result,
};
use diesel::prelude::*;

pub(crate) fn all<S: AsRef<str>>(
    connection: &mut SqliteConnection,
    id: S,
) -> Result<Vec<LineItemDate>> {
    let records = line_item_dates::table
        .filter(line_item_dates::quote_id.eq(&id.as_ref()))
        .get_results(connection)?;
    Ok(records)
}

pub(crate) fn read<S: AsRef<str>>(
    connection: &mut SqliteConnection,
    id: S,
) -> Result<LineItemDate> {
    let record = line_item_dates::table
        .filter(line_item_dates::id.eq(&id.as_ref()))
        .get_result(connection)?;
    Ok(record)
}

pub(crate) fn insert(
    connection: &mut SqliteConnection,
    form: &NewLineItemDateForm,
) -> Result<LineItemDate> {
    let record: LineItemDate = form.into();

    diesel::dsl::insert_into(line_item_dates::table)
        .values(&record)
        .execute(connection)?;

    Ok(record)
}

pub(crate) fn update(
    connection: &mut SqliteConnection,
    form: &EditLineItemDateForm,
) -> Result<LineItemDate> {
    let record: LineItemDate = form.into();

    diesel::dsl::update(line_item_dates::table)
        .set((
            line_item_dates::date.eq(&record.date),
            line_item_dates::updated_at.eq(&record.updated_at),
        ))
        .filter(line_item_dates::id.eq(&record.id))
        .execute(connection)?;

    read(connection, &record.id)
}

pub(crate) fn delete<S: AsRef<str>>(
    connection: &mut SqliteConnection,
    id: S,
) -> Result<LineItemDate> {
    let record = read(connection, &id)?;
    _ = connection.transaction::<_, _, _>(|tx| {
        line_items::query::delete_all_for_date(tx, &id)?;

        _ = diesel::dsl::delete(line_item_dates::table)
            .filter(line_item_dates::id.eq(&id.as_ref()))
            .execute(tx)?;

        Ok::<(), crate::error::AppError>(())
    });

    Ok(record)
}

pub(crate) fn delete_all_for_quote<S: AsRef<str>>(tx: &mut SqliteConnection, id: S) -> Result {
    line_items::query::delete_all_for_quote(tx, &id)?;

    _ = diesel::dsl::delete(line_item_dates::table)
        .filter(line_item_dates::quote_id.eq(&id.as_ref()))
        .execute(tx)?;

    Ok(())
}
