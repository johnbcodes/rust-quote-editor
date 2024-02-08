use crate::{
    forms::validate_date,
    quotes::model::QuoteWithTotal,
    schema::line_item_dates,
    time::{long_form, parse_date, short_form},
};
use diesel::prelude::*;
use time::{Date, OffsetDateTime};
use ulid::Ulid;

#[derive(Debug, Insertable, Queryable)]
pub struct LineItemDate {
    pub id: String,
    pub quote_id: String,
    pub date: Date,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl From<&EditLineItemDateForm> for LineItemDate {
    fn from(value: &EditLineItemDateForm) -> Self {
        let date = parse_date(&value.date);
        LineItemDate {
            id: value.id.clone(),
            quote_id: value.quote_id.clone(),
            date,
            created_at: OffsetDateTime::now_utc(),
            updated_at: OffsetDateTime::now_utc(),
        }
    }
}

impl From<&NewLineItemDateForm> for LineItemDate {
    fn from(value: &NewLineItemDateForm) -> Self {
        let date = parse_date(&value.date);
        LineItemDate {
            id: Ulid::new().to_string(),
            quote_id: value.quote_id.clone(),
            date,
            created_at: OffsetDateTime::now_utc(),
            updated_at: OffsetDateTime::now_utc(),
        }
    }
}

#[derive(Clone, Debug, FromForm)]
pub struct EditLineItemDateForm {
    #[field(validate = len(1..))]
    pub id: String,
    #[field(validate = len(1..))]
    pub quote_id: String,
    #[field(validate = validate_date())]
    pub date: String,
}

#[derive(Clone, Debug, FromForm)]
pub struct NewLineItemDateForm {
    #[field(validate = len(1..))]
    pub quote_id: String,
    #[field(validate = validate_date())]
    pub date: String,
}

#[derive(Debug, Default)]
pub struct LineItemDatePresenter {
    pub id: Option<String>,
    pub quote_id: String,
    pub date: Option<Date>,
}

impl LineItemDatePresenter {
    pub fn from_quote_with_total(quote: QuoteWithTotal) -> LineItemDatePresenter {
        LineItemDatePresenter {
            quote_id: quote.id,
            ..Default::default()
        }
    }

    pub fn id(&self) -> String {
        match &self.id {
            Some(id) => id.clone(),
            None => String::from("new"),
        }
    }

    pub fn dom_id(&self) -> String {
        format!("line_item_date_{}", &self.id())
    }

    pub fn edit_dom_id(&self) -> String {
        format!("edit_line_item_date_{}", &self.id())
    }

    pub fn date_long_form(&self) -> String {
        match self.date {
            Some(date) => long_form(date),
            None => "".to_string(),
        }
    }

    pub fn date_short_form(&self) -> String {
        match self.date {
            Some(date) => short_form(date),
            None => "".to_string(),
        }
    }
}

impl From<LineItemDate> for LineItemDatePresenter {
    fn from(value: LineItemDate) -> Self {
        LineItemDatePresenter {
            id: Some(value.id),
            quote_id: value.quote_id.to_string(),
            date: Some(value.date),
        }
    }
}

impl From<EditLineItemDateForm> for LineItemDatePresenter {
    fn from(value: EditLineItemDateForm) -> Self {
        let date = parse_date(&value.date);
        LineItemDatePresenter {
            id: Some(value.id),
            quote_id: value.quote_id,
            date: Some(date),
        }
    }
}

impl From<NewLineItemDateForm> for LineItemDatePresenter {
    fn from(value: NewLineItemDateForm) -> Self {
        let date = parse_date(&value.date);
        LineItemDatePresenter {
            id: None,
            quote_id: value.quote_id,
            date: Some(date),
        }
    }
}

#[derive(Debug, FromForm)]
pub(crate) struct DeleteForm {
    pub(crate) id: String,
}
