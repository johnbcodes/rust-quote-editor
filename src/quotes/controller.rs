use crate::{
    layout::{Flash, Layout},
    line_item_dates::{self, model::LineItemDatePresenter},
    line_items::{self, model::LineItemPresenter},
    quotes::{
        self,
        model::{DeleteForm, EditQuoteForm, NewQuoteForm, QuotePresenter},
        view::*,
    },
    rocket_ext::HtmxResponder,
    Db, Result,
};
use itertools::Itertools;
use rocket::{
    fairing::AdHoc,
    form::{Contextual, Form},
    http::Header,
    response::content::RawHtml,
};

pub(crate) fn stage() -> AdHoc {
    AdHoc::on_ignite("Quote Stage", |rocket| async {
        rocket.mount(
            "/quotes",
            routes![index, quote, show, new, create, edit, update, delete],
        )
    })
}

#[get("/")]
async fn index(db: Db) -> Result<RawHtml<String>> {
    let quotes = db
        .run(move |conn| {
            let records = quotes::query::all(conn)?
                .into_iter()
                .map(|record| record.into())
                .collect::<Vec<QuotePresenter>>();
            Result::Ok(records)
        })
        .await?;

    let template = Layout {
        head: markup::new! {
            title { "Quotes" }
        },
        body: Index { quotes },
    };

    Ok(RawHtml(template.to_string()))
}

#[get("/<id>")]
async fn quote(db: Db, id: String) -> Result<RawHtml<String>> {
    let quote = db
        .run(move |conn| {
            let quote = quotes::query::read(conn, &id)?;
            Result::Ok(quote)
        })
        .await?;

    let quote = Quote {
        quote: &quote.into(),
    };
    Ok(RawHtml(quote.to_string()))
}

#[get("/show/<id>")]
async fn show(db: Db, id: String) -> Result<RawHtml<String>> {
    let quote = db
        .run(move |conn| {
            let quote = quotes::query::read(conn, &id)?;
            Result::Ok(quote)
        })
        .await?;

    let quote_id = quote.id.clone();
    let line_item_dates = db
        .run(move |conn| {
            let records = line_item_dates::query::all(conn, quote_id)?
                .into_iter()
                .map(|record| record.into())
                .collect::<Vec<LineItemDatePresenter>>();
            Result::Ok(records)
        })
        .await?;

    let quote_id = quote.id.clone();
    let line_items = db
        .run(move |conn| {
            let index = line_items::query::all_for_quote(conn, quote_id)?
                .into_iter()
                .map(|record| record.into())
                .collect::<Vec<LineItemPresenter>>()
                .into_iter()
                .into_group_map_by(|line_item| line_item.line_item_date_id.clone());
            Result::Ok(index)
        })
        .await?;

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

    Ok(RawHtml(template.to_string()))
}

#[get("/new")]
async fn new() -> RawHtml<String> {
    RawHtml(NewForm {}.to_string())
}

#[post("/create", data = "<form>")]
async fn create(db: Db, form: Form<Contextual<'_, NewQuoteForm>>) -> Result<HtmxResponder> {
    print!("Form:\n{form:?}");
    match form.value {
        Some(ref quote_form) => {
            let quote_form = quote_form.clone();
            let quote = db
                .run(move |conn| {
                    let record = quotes::query::insert(conn, &quote_form)?;
                    Result::Ok(record)
                })
                .await?;

            let content = Create {
                quote: &quote.into(),
                message: "Quote was successfully created.",
            }
            .to_string();

            Ok(HtmxResponder::Ok(content))
        }
        None => {
            let template = NewFormWithErrors { form: &form };
            let content = template.to_string();
            Ok(HtmxResponder::Retarget {
                content,
                retarget: Header::new("HX-Retarget", "#quote_new".to_string()),
                reswap: Header::new("HX-Reswap", "outerhtml".to_string()),
            })
        }
    }
}

#[get("/edit/<id>")]
async fn edit(db: Db, id: String) -> Result<RawHtml<String>> {
    let quote = db
        .run(move |conn| {
            let quote = quotes::query::read(conn, &id)?;
            Result::Ok(quote)
        })
        .await?;

    Ok(RawHtml(
        EditForm {
            quote: &quote.into(),
        }
        .to_string(),
    ))
}

#[post("/update", data = "<form>")]
async fn update(db: Db, form: Form<Contextual<'_, EditQuoteForm>>) -> Result<RawHtml<String>> {
    match form.value {
        Some(ref quote_form) => {
            let quote_form = quote_form.clone();
            let quote = db
                .run(move |conn| {
                    let record = quotes::query::update(conn, &quote_form)?;
                    Result::Ok(record)
                })
                .await?;

            Ok(RawHtml(
                Update {
                    quote: &quote.into(),
                    message: "Quote was successfully updated.",
                }
                .to_string(),
            ))
        }
        None => {
            let template = EditFormWithErrors { form: &form };
            let html = template.to_string();
            Ok(RawHtml(html))
        }
    }
}

#[post("/delete", data = "<form>")]
async fn delete(db: Db, form: Form<DeleteForm>) -> Result<RawHtml<String>> {
    db.run(move |conn| {
        quotes::query::delete(conn, &form.id)?;
        Result::Ok(())
    })
    .await?;

    Ok(RawHtml(
        Flash {
            message: "Quote was successfully destroyed.",
        }
        .to_string(),
    ))
}
