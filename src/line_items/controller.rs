use crate::{
    line_item_dates,
    line_items::{
        self,
        model::{DeleteForm, EditLineItemForm, LineItemPresenter, NewLineItemForm},
        view::*,
    },
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
    AdHoc::on_ignite("LineItem Stage", |rocket| async {
        rocket.mount(
            "/line_items",
            routes![line_item, new, create, edit, update, delete],
        )
    })
}

#[get("/<id>")]
async fn line_item(db: Db, id: String) -> Result<RawHtml<String>> {
    let line_item = db
        .run(move |conn| {
            let line_item = line_items::query::read(conn, id)?;
            Result::Ok(line_item)
        })
        .await?;

    let template = LineItem {
        line_item: &line_item.into(),
    };
    Ok(RawHtml(template.to_string()))
}

#[get("/new/<line_item_date_id>")]
async fn new(db: Db, line_item_date_id: String) -> Result<RawHtml<String>> {
    let lid_id = line_item_date_id.clone();
    let quote = db
        .run(move |conn| {
            let quote = quotes::query::from_line_item_date_id(conn, &lid_id)?;
            Result::Ok(quote)
        })
        .await?;

    Ok(RawHtml(
        NewForm {
            line_item: &LineItemPresenter::from_line_item_date(line_item_date_id),
            quote: &quote.into(),
        }
        .to_string(),
    ))
}

#[post("/create", data = "<form>")]
async fn create(db: Db, form: Form<Contextual<'_, NewLineItemForm>>) -> Result<HtmxResponder> {
    print!("Form:\n{form:?}");
    match form.value {
        Some(ref li_form) => {
            let quote_id = li_form.quote_id.clone();
            let li_form = li_form.clone();
            let line_item = db
                .run(move |conn| {
                    let line_item = line_items::query::insert(conn, &li_form)?;
                    Result::Ok(line_item)
                })
                .await?;

            let quote = db
                .run(move |conn| {
                    let quote = quotes::query::read(conn, &quote_id)?;
                    Result::Ok(quote)
                })
                .await?;

            let content = Create {
                line_item: &line_item.into(),
                quote: &quote.into(),
                message: "Item was successfully created.",
            }
            .to_string();

            Ok(HtmxResponder::Ok(content))
        }
        None => {
            let template = NewFormWithErrors { form: &form };
            let content = template.to_string();
            let line_item_date_id = form.context.field_value("line_item_date_id").unwrap_or("");
            let retarget = format!("#line_item_date_{}_line_item_new", line_item_date_id);
            Ok(HtmxResponder::Retarget {
                content,
                retarget: Header::new("HX-Retarget", retarget),
                reswap: Header::new("HX-Reswap", "outerhtml".to_string()),
            })
        }
    }
}

#[get("/edit/<id>")]
async fn edit<'a>(db: Db, id: String) -> Result<RawHtml<String>> {
    let line_item = db
        .run(move |conn| {
            let line_item = line_items::query::read(conn, id)?;
            Result::Ok(line_item)
        })
        .await?;

    let lid_id = line_item.line_item_date_id.clone();
    let quote = db
        .run(move |conn| {
            let line_item_date = line_item_dates::query::read(conn, &lid_id)?;
            let quote = quotes::query::read(conn, &line_item_date.quote_id)?;
            Result::Ok(quote)
        })
        .await?;

    Ok(RawHtml(
        EditForm {
            line_item: &line_item.into(),
            quote: &quote.into(),
        }
        .to_string(),
    ))
}

#[post("/update", data = "<form>")]
async fn update(db: Db, form: Form<Contextual<'_, EditLineItemForm>>) -> Result<RawHtml<String>> {
    print!("Form:\n{form:?}");
    match form.value {
        Some(ref li_form) => {
            let quote_id = li_form.quote_id.clone();
            let li_form = li_form.clone();
            let line_item = db
                .run(move |conn| {
                    let line_item = line_items::query::update(conn, &li_form)?;
                    Result::Ok(line_item)
                })
                .await?;

            let quote = db
                .run(move |conn| {
                    let quote = quotes::query::read(conn, &quote_id)?;
                    Result::Ok(quote)
                })
                .await?;

            let content = Update {
                line_item: &line_item.into(),
                quote: &quote.into(),
                message: "Item was successfully updated.",
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
            let line_item = line_items::query::delete(conn, &form.id)?;
            let quote = quotes::query::from_line_item_date_id(conn, &line_item.line_item_date_id)?;
            Result::Ok(quote)
        })
        .await?;

    Ok(RawHtml(
        Destroy {
            quote: &quote.into(),
            message: "Item was successfully destroyed.",
        }
        .to_string(),
    ))
}
