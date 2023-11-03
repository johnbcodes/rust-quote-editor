use crate::schema::quotes;
use currency_rs::Currency;
use diesel::prelude::*;
use diesel::sql_types::*;
use serde::Deserialize;
use time::OffsetDateTime;
use ulid::Ulid;
use validator::Validate;

#[derive(Debug, QueryableByName)]
pub struct QuoteWithTotal {
    #[diesel(sql_type = Text)]
    pub id: String,
    #[diesel(sql_type = Text)]
    pub name: String,
    #[diesel(sql_type = currency_rs::diesel2::sqlite::sql_types::Currency)]
    pub total: Currency,
    #[diesel(sql_type = TimestamptzSqlite)]
    pub created_at: OffsetDateTime,
    #[diesel(sql_type = TimestamptzSqlite)]
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Insertable, Queryable)]
#[diesel(table_name = quotes)]
pub struct Quote {
    pub id: String,
    pub name: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl From<&QuoteForm> for Quote {
    fn from(value: &QuoteForm) -> Self {
        Quote {
            id: value.id.clone().unwrap_or(Ulid::new().to_string()),
            name: value.name.clone(),
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
            total: Currency::new_float(0f64, None),
        }
    }
}

impl From<QuoteWithTotal> for QuotePresenter {
    fn from(value: QuoteWithTotal) -> Self {
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
