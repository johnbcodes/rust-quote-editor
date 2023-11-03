use crate::{
    line_item_dates,
    line_items::{
        self,
        model::{DeleteForm, LineItemForm, LineItemPresenter},
        view::{Create, Destroy, Form, Update},
    },
    quotes, Result,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse},
};
use diesel::prelude::SqliteConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use hotwire_turbo_axum::TurboStream;
use std::time::Instant;
use tracing::info;
use validator::Validate;

pub(crate) async fn new(
    State(pool): State<Pool<ConnectionManager<SqliteConnection>>>,
    Path(line_item_date_id): Path<String>,
) -> Result<impl IntoResponse> {
    let start = Instant::now();
    let quote = quotes::query::from_line_item_date_id(&pool, &line_item_date_id).await?;
    let duration = start.elapsed().as_micros();
    info!("lid - read duration: {duration} μs");
    Ok(Html(
        Form {
            line_item: &LineItemPresenter::from_line_item_date(line_item_date_id),
            quote: &quote.into(),
            action: "create",
            error_message: None,
        }
        .to_string(),
    ))
}

pub(crate) async fn create(
    State(pool): State<Pool<ConnectionManager<SqliteConnection>>>,
    axum::Form(form): axum::Form<LineItemForm>,
) -> Result<impl IntoResponse> {
    let result = form.validate();
    match result {
        Ok(_) => {
            let start = Instant::now();
            let line_item = line_items::query::insert(&pool, &form).await?;
            let duration = start.elapsed().as_micros();
            info!("li  - insert duration: {duration} μs");
            let start = Instant::now();
            let quote = quotes::query::read(&pool, &form.quote_id).await?;
            let duration = start.elapsed().as_micros();
            info!("quo - read duration: {duration} μs");
            Ok(TurboStream(
                Create {
                    line_item: &line_item.into(),
                    quote: &quote.into(),
                    message: "Item was successfully created.",
                }
                .to_string(),
            )
            .into_response())
        }
        Err(errors) => {
            info!("ValidationErrors:\n{:?}", errors);
            let start = Instant::now();
            let quote = quotes::query::read(&pool, &form.quote_id).await?;
            let duration = start.elapsed().as_micros();
            info!("quo - read duration: {duration} μs");
            let error_message = String::from("Test");
            Ok((
                StatusCode::UNPROCESSABLE_ENTITY,
                Html(
                    Form {
                        line_item: &form.into(),
                        quote: &quote.into(),
                        action: "create",
                        error_message: Some(error_message),
                    }
                    .to_string(),
                ),
            )
                .into_response())
        }
    }
}

pub(crate) async fn edit(
    State(pool): State<Pool<ConnectionManager<SqliteConnection>>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse> {
    let start = Instant::now();
    let line_item = line_items::query::read(&pool, id).await?;
    let duration = start.elapsed().as_micros();
    info!("li  - read duration: {duration} μs");
    let start = Instant::now();
    let line_item_date = line_item_dates::query::read(&pool, &line_item.line_item_date_id).await?;
    let duration = start.elapsed().as_micros();
    info!("lid - read duration: {duration} μs");
    let start = Instant::now();
    let quote = quotes::query::read(&pool, &line_item_date.quote_id).await?;
    let duration = start.elapsed().as_micros();
    info!("quo - read duration: {duration} μs");
    Ok(Html(
        Form {
            line_item: &line_item.into(),
            quote: &quote.into(),
            action: "update",
            error_message: None,
        }
        .to_string(),
    ))
}

pub(crate) async fn update(
    State(pool): State<Pool<ConnectionManager<SqliteConnection>>>,
    axum::Form(form): axum::Form<LineItemForm>,
) -> Result<impl IntoResponse> {
    let result = form.validate();
    match result {
        Ok(_) => {
            let start = Instant::now();
            let line_item = line_items::query::update(&pool, &form).await?;
            let duration = start.elapsed().as_micros();
            info!("li  - update duration: {duration} μs");
            let start = Instant::now();
            let quote =
                quotes::query::from_line_item_date_id(&pool, &form.line_item_date_id).await?;
            info!("Quote total after update: {}", quote.total);
            let duration = start.elapsed().as_micros();
            info!("quo - read duration: {duration} μs");
            Ok(TurboStream(
                Update {
                    line_item: &line_item.into(),
                    quote: &quote.into(),
                    message: "Item was successfully updated.",
                }
                .to_string(),
            )
            .into_response())
        }
        Err(errors) => {
            info!("ValidationErrors:\n{:?}", errors);
            let start = Instant::now();
            let quote =
                quotes::query::from_line_item_date_id(&pool, &form.line_item_date_id).await?;
            let duration = start.elapsed().as_micros();
            info!("quo - read duration: {duration} μs");
            let error_message = String::from("Test");
            Ok((
                StatusCode::UNPROCESSABLE_ENTITY,
                Html(
                    Form {
                        line_item: &form.into(),
                        quote: &quote.into(),
                        action: "update",
                        error_message: Some(error_message),
                    }
                    .to_string(),
                ),
            )
                .into_response())
        }
    }
}

pub(crate) async fn delete(
    State(pool): State<Pool<ConnectionManager<SqliteConnection>>>,
    axum::Form(form): axum::Form<DeleteForm>,
) -> Result<impl IntoResponse> {
    let start = Instant::now();
    let line_item = line_items::query::delete(&pool, &form.id).await?;
    let duration = start.elapsed().as_micros();
    info!("li  - delete duration: {duration} μs");
    let start = Instant::now();
    let quote = quotes::query::from_line_item_date_id(&pool, &line_item.line_item_date_id).await?;
    let duration = start.elapsed().as_micros();
    info!("quo - read duration: {duration} μs");
    Ok(TurboStream(
        Destroy {
            line_item: &line_item.into(),
            quote: &quote.into(),
            message: "Item was successfully destroyed.",
        }
        .to_string(),
    )
    .into_response())
}
