use crate::{
    quotes::model::Quote,
    time::{long_form, parse_date, short_form, DATE_REGEX},
};
use nanoid::nanoid;
use serde::Deserialize;
use time::{Date, OffsetDateTime};
use validator::Validate;

#[derive(Debug)]
pub struct LineItemDate {
    pub id: String,
    pub quote_id: String,
    pub date: Date,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl From<&LineItemDateForm> for LineItemDate {
    fn from(value: &LineItemDateForm) -> Self {
        let date = parse_date(&value.date);
        LineItemDate {
            id: value.id.clone().unwrap_or(nanoid!()),
            quote_id: value.quote_id.clone(),
            date,
            created_at: OffsetDateTime::now_utc(),
            updated_at: OffsetDateTime::now_utc(),
        }
    }
}

#[derive(Debug, Deserialize, Validate)]
pub(crate) struct LineItemDateForm {
    #[validate(length(min = 1, message = "can't be blank"))]
    pub(crate) id: Option<String>,
    #[validate(length(min = 1, message = "can't be blank"))]
    pub(crate) quote_id: String,
    #[validate(regex(path = "DATE_REGEX"))]
    pub(crate) date: String,
}

#[derive(Debug, Default)]
pub struct LineItemDatePresenter {
    pub id: Option<String>,
    pub quote_id: String,
    pub date: Option<Date>,
}

impl LineItemDatePresenter {
    pub fn from_quote(quote: Quote) -> LineItemDatePresenter {
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

impl From<LineItemDateForm> for LineItemDatePresenter {
    fn from(value: LineItemDateForm) -> Self {
        let date = parse_date(&value.date);
        LineItemDatePresenter {
            id: value.id,
            quote_id: value.quote_id,
            date: Some(date),
        }
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct DeleteForm {
    pub(crate) id: String,
}
