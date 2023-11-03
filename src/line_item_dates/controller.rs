use crate::{
    line_item_dates::{
        self,
        model::{DeleteForm, LineItemDateForm, LineItemDatePresenter},
        view::{Create, Destroy, Form, Update},
    },
    line_items::{self, model::LineItemPresenter},
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
    Path(quote_id): Path<String>,
) -> Result<impl IntoResponse> {
    let start = Instant::now();
    let quote = quotes::query::read(&pool, quote_id).await?;
    let duration = start.elapsed().as_micros();
    info!("quo - read duration: {duration} μs");
    let line_item_date = LineItemDatePresenter::from_quote_with_total(quote);
    let duration = start.elapsed().as_micros();
    info!("lid - read duration: {duration} μs");
    Ok(Html(
        Form {
            dom_id: &line_item_date.dom_id(),
            line_item_date: &line_item_date,
            action: "create",
            error_message: None,
        }
        .to_string(),
    ))
}

pub(crate) async fn create(
    State(pool): State<Pool<ConnectionManager<SqliteConnection>>>,
    axum::Form(form): axum::Form<LineItemDateForm>,
) -> Result<impl IntoResponse> {
    let result = form.validate();
    match result {
        Ok(_) => {
            let start = Instant::now();
            let line_item_date = line_item_dates::query::insert(&pool, &form).await?;
            let duration = start.elapsed().as_micros();
            info!("lid - insert duration: {duration} μs");
            Ok(TurboStream(
                Create {
                    line_item_date: &line_item_date.into(),
                    line_items: &Vec::new(),
                    message: "Date was successfully created.",
                }
                .to_string(),
            )
            .into_response())
        }
        Err(errors) => {
            info!("ValidationErrors:\n{:?}", errors);
            let error_message = String::from("Test");
            let line_item_date: &LineItemDatePresenter = &form.into();
            Ok((
                StatusCode::UNPROCESSABLE_ENTITY,
                Html(
                    Form {
                        dom_id: &line_item_date.dom_id(),
                        line_item_date,
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
    let record = line_item_dates::query::read(&pool, id).await?;
    let duration = start.elapsed().as_micros();
    info!("lid - read duration: {duration} μs");
    let line_item_date: &LineItemDatePresenter = &record.into();
    Ok(Html(
        Form {
            dom_id: &line_item_date.edit_dom_id(),
            line_item_date,
            action: "update",
            error_message: None,
        }
        .to_string(),
    ))
}

pub(crate) async fn update(
    State(pool): State<Pool<ConnectionManager<SqliteConnection>>>,
    axum::Form(form): axum::Form<LineItemDateForm>,
) -> Result<impl IntoResponse> {
    let result = form.validate();
    match result {
        Ok(_) => {
            let start = Instant::now();
            let line_item_date = line_item_dates::query::update(&pool, &form).await?;
            let duration = start.elapsed().as_micros();
            info!("lid - update duration: {duration} μs");
            let start = Instant::now();
            let line_items = line_items::query::all_for_line_item_date(&pool, &line_item_date.id)
                .await?
                .into_iter()
                .map(|record| record.into())
                .collect::<Vec<LineItemPresenter>>();
            let duration = start.elapsed().as_micros();
            info!("li - read all duration: {duration} μs");
            Ok(TurboStream(
                Update {
                    line_item_date: &line_item_date.into(),
                    line_items: &line_items,
                    message: "Date was successfully updated.",
                }
                .to_string(),
            )
            .into_response())
        }
        Err(errors) => {
            info!("ValidationErrors:\n{:?}", errors);
            let error_message = String::from("Test");
            let line_item_date: &LineItemDatePresenter = &form.into();
            Ok((
                StatusCode::UNPROCESSABLE_ENTITY,
                Html(
                    Form {
                        dom_id: &line_item_date.edit_dom_id(),
                        line_item_date,
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
    let line_item_date = line_item_dates::query::delete(&pool, &form.id).await?;
    let duration = start.elapsed().as_micros();
    info!("lid - delete duration: {duration} μs");
    let start = Instant::now();
    let quote = quotes::query::read(&pool, &line_item_date.quote_id).await?;
    let duration = start.elapsed().as_micros();
    info!("quo - read duration: {duration} μs");
    Ok(TurboStream(
        Destroy {
            line_item_date: &line_item_date.into(),
            quote: &quote.into(),
            message: "Date was successfully destroyed.",
        }
        .to_string(),
    )
    .into_response())
}
