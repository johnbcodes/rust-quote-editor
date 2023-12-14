use crate::{
    layout::{Flash, Layout},
    line_item_dates::{self, model::LineItemDatePresenter},
    line_items::{self, model::LineItemPresenter},
    quotes::{
        self,
        model::{DeleteForm, QuoteForm, QuotePresenter},
        view::{Create, EditForm, Index, NewForm, Quote, Show, Update},
    },
    Result,
};
use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse},
};
use diesel::prelude::SqliteConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use itertools::Itertools;
use std::time::Instant;
use tracing::info;
use validator::Validate;

pub(crate) async fn index(
    State(pool): State<Pool<ConnectionManager<SqliteConnection>>>,
) -> Result<Html<String>> {
    let start = Instant::now();
    let quotes = quotes::query::all(&pool)
        .await?
        .into_iter()
        .map(|record| record.into())
        .collect::<Vec<QuotePresenter>>();
    let duration = start.elapsed().as_micros();
    info!("quo - read duration: {duration} μs");

    let template = Layout {
        head: markup::new! {
            title { "Quotes" }
        },
        body: Index { quotes },
    };

    Ok(Html(template.to_string()))
}

pub(crate) async fn quote(
    State(pool): State<Pool<ConnectionManager<SqliteConnection>>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse> {
    let start = Instant::now();
    let quote = quotes::query::read(&pool, &id).await?;
    let duration = start.elapsed().as_micros();
    info!("quo - read duration: {duration} μs");

    let quote = Quote {
        quote: &quote.into(),
    };
    Ok(Html(quote.to_string()))
}

pub(crate) async fn show(
    State(pool): State<Pool<ConnectionManager<SqliteConnection>>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse> {
    let start = Instant::now();
    let quote = quotes::query::read(&pool, &id).await?;
    let duration = start.elapsed().as_micros();
    info!("quo - read duration: {duration} μs");
    let start = Instant::now();
    let line_item_dates = line_item_dates::query::all(&pool, &quote.id)
        .await?
        .into_iter()
        .map(|record| record.into())
        .collect::<Vec<LineItemDatePresenter>>();
    let duration = start.elapsed().as_micros();
    info!("lid - read all duration: {duration} μs");
    let start = Instant::now();
    let line_items = line_items::query::all_for_quote(&pool, &quote.id)
        .await?
        .into_iter()
        .map(|record| record.into())
        .collect::<Vec<LineItemPresenter>>()
        .into_iter()
        .into_group_map_by(|line_item| line_item.line_item_date_id.clone());
    let duration = start.elapsed().as_micros();
    info!("li  - read all duration: {duration} μs");

    let quote_name = quote.name.clone();
    let template = Layout {
        head: markup::new! {
            title { {format!("Quote {quote_name}")} }
        },
        body: Show {
            quote: &quote.into(),
            line_item_dates: &line_item_dates,
            line_items: &line_items,
        },
    };

    Ok(Html(template.to_string()))
}

pub(crate) async fn new() -> impl IntoResponse {
    Html(
        NewForm {
            quote: &QuotePresenter::default(),
            error_message: None,
        }
        .to_string(),
    )
}

pub(crate) async fn create(
    State(pool): State<Pool<ConnectionManager<SqliteConnection>>>,
    axum::Form(form): axum::Form<QuoteForm>,
) -> Result<impl IntoResponse> {
    let result = form.validate();
    match result {
        Ok(_) => {
            let start = Instant::now();
            let quote = quotes::query::insert(&pool, &form).await?;
            let duration = start.elapsed().as_micros();
            info!("quo - insert duration: {duration} μs");
            Ok(Html(
                Create {
                    quote: &quote.into(),
                    message: "Quote was successfully created.",
                }
                .to_string(),
            )
            .into_response())
        }
        Err(errors) => {
            info!("ValidationErrors:\n{:?}", errors);
            let error_message = String::from("Test");
            Ok(Html(
                NewForm {
                    quote: &form.into(),
                    error_message: Some(error_message),
                }
                .to_string(),
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
    let quote = quotes::query::read(&pool, &id).await?;
    let duration = start.elapsed().as_micros();
    info!("quo - read duration: {duration} μs");
    Ok(Html(
        EditForm {
            quote: &quote.into(),
            error_message: None,
        }
        .to_string(),
    ))
}

pub(crate) async fn update(
    State(pool): State<Pool<ConnectionManager<SqliteConnection>>>,
    axum::Form(form): axum::Form<QuoteForm>,
) -> Result<impl IntoResponse> {
    let result = form.validate();
    match result {
        Ok(_) => {
            let start = Instant::now();
            let quote = quotes::query::update(&pool, &form).await?;
            let duration = start.elapsed().as_micros();
            info!("quo - update duration: {duration} μs");
            Ok(Html(
                Update {
                    quote: &quote.into(),
                    message: "Quote was successfully updated.",
                }
                .to_string(),
            )
            .into_response())
        }
        Err(errors) => {
            info!("ValidationErrors:\n{:?}", errors);
            let error_message = String::from("Test");
            Ok(Html(
                EditForm {
                    quote: &form.into(),
                    error_message: Some(error_message),
                }
                .to_string(),
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
    quotes::query::delete(&pool, &form.id).await?;
    let duration = start.elapsed().as_micros();
    info!("quo - delete duration: {duration} μs");
    Ok(Html(
        Flash {
            message: "Quote was successfully destroyed.",
        }
        .to_string(),
    )
    .into_response())
}
