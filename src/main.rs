#![warn(clippy::all)]
#![deny(unreachable_pub, private_in_public)]
#![forbid(unsafe_code)]

mod assets;
mod currency;
mod error;
mod layout;
pub mod line_item_dates;
pub mod line_items;
mod migrations;
pub mod quotes;
mod time;

use assets::asset_handler;
use axum::{
    handler::HandlerWithoutStateExt,
    response::Redirect,
    routing::{get, post, Router},
};
use dotenvy::dotenv;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::OpenFlags as of;
use std::env;
use tower_http::trace::{DefaultOnResponse, TraceLayer};
use tower_http::LatencyUnit;
use tracing::{info, Level};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub(crate) type Result<T = ()> = std::result::Result<T, error::AppError>;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let rust_log = env::var("RUST_LOG").unwrap_or_else(|_| {
        let value = "INFO,tower_http=info";
        env::set_var("RUST_LOG", value);
        value.into()
    });
    println!("RUST_LOG={rust_log}");

    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(fmt::layer())
        .init();

    let db_url = env::var("DATABASE_FILE").unwrap();
    println!("DATABASE_FILE={db_url}");

    let manager = SqliteConnectionManager::file(db_url.as_str())
        .with_flags(of::SQLITE_OPEN_URI | of::SQLITE_OPEN_CREATE | of::SQLITE_OPEN_READ_WRITE)
        .with_init(|conn| conn.pragma_update(None, "journal_mode", "wal"))
        .with_init(|conn| conn.pragma_update(None, "synchronous", "normal"))
        .with_init(|conn| conn.pragma_update(None, "foreign_keys", "on"));
    let pool = Pool::builder()
        .max_size(10)
        .build(manager)
        .expect("unable to build pool");

    let mut conn = pool.get().unwrap();
    migrations::MIGRATIONS.to_latest(&mut conn).unwrap();
    drop(conn);

    let trace_layer = TraceLayer::new_for_http().on_response(
        DefaultOnResponse::new()
            .level(Level::INFO)
            .latency_unit(LatencyUnit::Micros),
    );

    let app = Router::new()
        .route("/", get(|| async { Redirect::to("/quotes") }))
        .route("/quotes", get(quotes::controller::index))
        .route("/quotes/show/:id", get(quotes::controller::show))
        .route("/quotes/new", get(quotes::controller::new))
        .route("/quotes/create", post(quotes::controller::create))
        .route("/quotes/edit/:id", get(quotes::controller::edit))
        .route("/quotes/update", post(quotes::controller::update))
        .route("/quotes/delete", post(quotes::controller::delete))
        .route_service("/dist/*file", asset_handler.into_service())
        .route(
            "/line_item_dates/new/:quote_id",
            get(line_item_dates::controller::new),
        )
        .route(
            "/line_item_dates/create",
            post(line_item_dates::controller::create),
        )
        .route(
            "/line_item_dates/edit/:id",
            get(line_item_dates::controller::edit),
        )
        .route(
            "/line_item_dates/update",
            post(line_item_dates::controller::update),
        )
        .route(
            "/line_item_dates/delete",
            post(line_item_dates::controller::delete),
        )
        .route(
            "/line_items/new/:line_item_date_id",
            get(line_items::controller::new),
        )
        .route("/line_items/create", post(line_items::controller::create))
        .route("/line_items/edit/:id", get(line_items::controller::edit))
        .route("/line_items/update", post(line_items::controller::update))
        .route("/line_items/delete", post(line_items::controller::delete))
        .with_state(pool)
        .layer(trace_layer)
        .fallback_service(asset_handler.into_service());

    let addr = "[::]:8080".parse().unwrap();
    info!("listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
