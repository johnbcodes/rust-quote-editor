#![warn(clippy::all)]
#![allow(clippy::blocks_in_conditions)] // Until https://github.com/rwf2/Rocket/issues/2655 is released
#![allow(clippy::needless_lifetimes)] // Until clippy is fixed https://github.com/rust-lang/rust-clippy/issues/13811
#![deny(unreachable_pub, private_bounds, private_interfaces)]
#![forbid(unsafe_code)]

#[macro_use]
extern crate rocket;

mod assets;
mod currency;
mod error;
mod forms;
pub mod layout;
pub mod line_item_dates;
pub mod line_items;
pub mod quotes;
mod rocket_ext;
mod schema;
mod time;

use diesel::sqlite::SqliteConnection;
use rocket::response::Redirect;
use rocket::{Build, Rocket, fairing::AdHoc};
use rocket_sync_db_pools::database;

#[database("demo")]
struct Db(SqliteConnection);

pub(crate) type Result<T = ()> = std::result::Result<T, error::AppError>;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(AdHoc::on_ignite("Diesel SQLite Stage", |rocket| async {
            rocket
                .attach(Db::fairing())
                .attach(AdHoc::on_ignite("Diesel Migrations", run_migrations))
        }))
        .mount("/", routes![index])
        .attach(quotes::controller::stage())
        .attach(line_item_dates::controller::stage())
        .attach(line_items::controller::stage())
        .attach(assets::stage())
}

#[get("/")]
fn index() -> Redirect {
    Redirect::to(uri!("/quotes"))
}

async fn run_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};

    const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

    Db::get_one(&rocket)
        .await
        .expect("failure obtaining database connection")
        .run(|conn| {
            conn.run_pending_migrations(MIGRATIONS)
                .expect("failure running diesel migrations");
        })
        .await;

    rocket
}
