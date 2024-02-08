use crate::{
    line_item_dates::{
        self,
        model::{DeleteForm, EditLineItemDateForm, LineItemDatePresenter, NewLineItemDateForm},
        view::*,
    },
    line_items::{self, model::LineItemPresenter},
    quotes,
    rocket_ext::HtmxResponder,
    Db, Result,
};
use rocket::{
    fairing::AdHoc,
    form::{Contextual, Form},
    http::Header,
    response::content::RawHtml,
};

pub(crate) fn stage() -> AdHoc {
    AdHoc::on_ignite("LineItemDate Stage", |rocket| async {
        rocket.mount(
            "/line_item_dates",
            routes![line_item_date, new, create, edit, update, delete],
        )
    })
}

#[get("/<id>")]
async fn line_item_date(db: Db, id: String) -> Result<RawHtml<String>> {
    let record = db
        .run(move |conn| {
            let quote = line_item_dates::query::read(conn, &id)?;
            Result::Ok(quote)
        })
        .await?;

    let template = LineItemDateInfo {
        line_item_date: &record.into(),
    };
    Ok(RawHtml(template.to_string()))
}

#[get("/new/<quote_id>")]
pub(crate) async fn new(quote_id: &str) -> Result<RawHtml<String>> {
    let template = NewForm { quote_id };
    let html = template.to_string();
    Ok(RawHtml(html))
}

#[post("/create", data = "<form>")]
pub(crate) async fn create(
    db: Db,
    form: Form<Contextual<'_, NewLineItemDateForm>>,
) -> Result<HtmxResponder> {
    print!("Form:\n{form:?}");
    match form.value {
        Some(ref lid_form) => {
            let lid_form = lid_form.clone();
            let line_item_date = db
                .run(move |conn| {
                    let record = line_item_dates::query::insert(conn, &lid_form)?;
                    Result::Ok(record)
                })
                .await?;

            let content = Create {
                line_item_date: &line_item_date.into(),
                line_items: &Vec::new(),
                message: "Date was successfully created.",
            }
            .to_string();

            Ok(HtmxResponder::Ok(content))
        }
        None => {
            let template = NewFormWithErrors { form: &form };
            let content = template.to_string();
            Ok(HtmxResponder::Retarget {
                content,
                retarget: Header::new("HX-Retarget", "#line_item_date_new".to_string()),
                reswap: Header::new("HX-Reswap", "outerhtml".to_string()),
            })
        }
    }
}

#[get("/edit/<id>")]
pub(crate) async fn edit(db: Db, id: String) -> Result<RawHtml<String>> {
    let record = db
        .run(move |conn| {
            let quote = line_item_dates::query::read(conn, &id)?;
            Result::Ok(quote)
        })
        .await?;

    let line_item_date: &LineItemDatePresenter = &record.into();
    let template = EditForm { line_item_date };
    let html = template.to_string();
    Ok(RawHtml(html))
}

#[post("/update", data = "<form>")]
pub(crate) async fn update(
    db: Db,
    form: Form<Contextual<'_, EditLineItemDateForm>>,
) -> Result<RawHtml<String>> {
    print!("Form:\n{form:?}");
    match form.value {
        Some(ref lid_form) => {
            let lid_form = lid_form.clone();
            let line_item_date = db
                .run(move |conn| {
                    let record = line_item_dates::query::update(conn, &lid_form)?;
                    Result::Ok(record)
                })
                .await?;

            let lid_id = line_item_date.id.clone();
            let line_items = db
                .run(move |conn| {
                    let line_items = line_items::query::all_for_line_item_date(conn, &lid_id)?
                        .into_iter()
                        .map(|record| record.into())
                        .collect::<Vec<LineItemPresenter>>();
                    Result::Ok(line_items)
                })
                .await?;

            let content = Update {
                line_item_date: &line_item_date.into(),
                line_items: &line_items,
                message: "Date was successfully updated.",
            }
            .to_string();

            Ok(RawHtml(content))
        }
        None => {
            let template = EditFormWithErrors { form: &form };
            let content = template.to_string();
            Ok(RawHtml(content))
        }
    }
}

#[post("/delete", data = "<form>")]
async fn delete(db: Db, form: Form<DeleteForm>) -> Result<RawHtml<String>> {
    let quote = db
        .run(move |conn| {
            let line_item_date = line_item_dates::query::delete(conn, &form.id)?;
            let quote = quotes::query::read(conn, &line_item_date.quote_id)?;
            Result::Ok(quote)
        })
        .await?;

    Ok(RawHtml(
        Destroy {
            quote: &quote.into(),
            message: "Date was successfully destroyed.",
        }
        .to_string(),
    ))
}
