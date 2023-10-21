use currency_rs::Currency;
use nanoid::nanoid;
use serde::Deserialize;
use time::OffsetDateTime;
use validator::Validate;

#[derive(Debug)]
pub struct Quote {
    pub id: String,
    pub name: String,
    pub total: Currency,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl From<&QuoteForm> for Quote {
    fn from(value: &QuoteForm) -> Self {
        Quote {
            id: value.id.clone().unwrap_or(nanoid!()),
            name: value.name.clone(),
            total: Currency::new_float(0f64, None),
            created_at: OffsetDateTime::now_utc(),
            updated_at: OffsetDateTime::now_utc(),
        }
    }
}

#[derive(Debug, Deserialize, Validate)]
pub(crate) struct QuoteForm {
    #[validate(length(min = 1, message = "can't be blank"))]
    pub(crate) id: Option<String>,
    #[validate(length(min = 1, message = "Can't be blank"))]
    pub(crate) name: String,
}

#[derive(Debug)]
pub struct QuotePresenter {
    pub id: Option<String>,
    pub name: String,
    pub total: Currency,
}

impl QuotePresenter {
    pub fn id(&self) -> String {
        match &self.id {
            Some(id) => id.clone(),
            None => String::from("new"),
        }
    }

    pub fn dom_id(&self) -> String {
        format!("quote_{}", &self.id())
    }

    pub fn total_dom_id(&self) -> String {
        format!("quote_total_{}", &self.id())
    }
}

impl Default for QuotePresenter {
    fn default() -> Self {
        QuotePresenter {
            id: None,
            name: String::from(""),
            total: Currency::new_float(0f64, None),
        }
    }
}

impl From<Quote> for QuotePresenter {
    fn from(value: Quote) -> Self {
        QuotePresenter {
            id: Some(value.id),
            name: value.name,
            total: value.total,
        }
    }
}

impl From<QuoteForm> for QuotePresenter {
    fn from(value: QuoteForm) -> Self {
        QuotePresenter {
            id: value.id,
            name: value.name,
            total: Currency::new_float(0f64, None),
        }
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct DeleteForm {
    pub(crate) id: String,
}
