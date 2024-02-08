use crate::schema::quotes;
use currency_rs::Currency;
use diesel::prelude::*;
use diesel::sql_types::*;
use time::OffsetDateTime;
use ulid::Ulid;

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

impl From<&NewQuoteForm> for Quote {
    fn from(value: &NewQuoteForm) -> Self {
        Quote {
            id: Ulid::new().to_string(),
            name: value.name.clone(),
            created_at: OffsetDateTime::now_utc(),
            updated_at: OffsetDateTime::now_utc(),
        }
    }
}

impl From<&EditQuoteForm> for Quote {
    fn from(value: &EditQuoteForm) -> Self {
        Quote {
            id: value.id.clone(),
            name: value.name.clone(),
            created_at: OffsetDateTime::now_utc(),
            updated_at: OffsetDateTime::now_utc(),
        }
    }
}

#[derive(Clone, Debug, FromForm)]
pub struct NewQuoteForm {
    #[field(validate = len(1..).or_else(msg!("Please enter a name")))]
    pub(crate) name: String,
}

#[derive(Clone, Debug, FromForm)]
pub struct EditQuoteForm {
    #[field(validate = len(1..))]
    pub(crate) id: String,
    #[field(validate = len(1..).or_else(msg!("Please enter a name")))]
    pub(crate) name: String,
}

#[derive(Clone, Debug)]
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

impl From<NewQuoteForm> for QuotePresenter {
    fn from(value: NewQuoteForm) -> Self {
        QuotePresenter {
            id: None,
            name: value.name,
            total: Currency::new_float(0f64, None),
        }
    }
}

impl From<EditQuoteForm> for QuotePresenter {
    fn from(value: EditQuoteForm) -> Self {
        QuotePresenter {
            id: Some(value.id),
            name: value.name,
            total: Currency::new_float(0f64, None),
        }
    }
}

#[derive(Clone, Debug, FromForm)]
pub(crate) struct DeleteForm {
    pub(crate) id: String,
}
