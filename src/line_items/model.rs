use crate::currency::FORM_CURRENCY_REGEX;
use crate::schema::line_items;
use currency_rs::Currency;
use diesel::prelude::*;
use serde::Deserialize;
use time::OffsetDateTime;
use ulid::Ulid;
use validator::Validate;

#[derive(Debug, Insertable, Queryable, Selectable)]
pub(crate) struct LineItem {
    pub(crate) id: String,
    pub(crate) line_item_date_id: String,
    pub(crate) name: String,
    pub(crate) description: Option<String>,
    pub(crate) quantity: i32,
    pub(crate) unit_price: Currency,
    pub(crate) created_at: OffsetDateTime,
    pub(crate) updated_at: OffsetDateTime,
}

impl From<&LineItemForm> for LineItem {
    fn from(value: &LineItemForm) -> Self {
        let description = value.description.clone().unwrap_or(String::from(""));
        let description = if description.is_empty() {
            None
        } else {
            Some(description)
        };
        LineItem {
            id: value.id.clone().unwrap_or(Ulid::new().to_string()),
            line_item_date_id: value.line_item_date_id.clone(),
            name: value.name.clone(),
            description,
            quantity: value.quantity,
            unit_price: Currency::new_string(value.unit_price.as_str(), None)
                .unwrap_or(Currency::new_float(0f64, None)),
            created_at: OffsetDateTime::now_utc(),
            updated_at: OffsetDateTime::now_utc(),
        }
    }
}

#[derive(Debug, Deserialize, Validate)]
pub(crate) struct LineItemForm {
    #[validate(length(min = 1, message = "can't be blank"))]
    pub(crate) id: Option<String>,
    #[validate(length(min = 1, message = "can't be blank"))]
    pub(crate) line_item_date_id: String,
    #[validate(length(min = 1, message = "can't be blank"))]
    pub(crate) quote_id: String,
    #[validate(length(min = 1, message = "can't be blank"))]
    pub(crate) name: String,
    pub(crate) description: Option<String>,
    pub(crate) quantity: i32,
    #[validate(regex(path = "FORM_CURRENCY_REGEX"))]
    pub(crate) unit_price: String,
}

#[derive(Debug)]
pub struct LineItemPresenter {
    pub id: Option<String>,
    pub line_item_date_id: String,
    pub name: String,
    pub description: String,
    pub quantity: String,
    pub unit_price: Currency,
}

impl LineItemPresenter {
    pub fn from_line_item_date(line_item_date_id: String) -> LineItemPresenter {
        LineItemPresenter {
            id: Default::default(),
            line_item_date_id,
            name: Default::default(),
            description: Default::default(),
            quantity: Default::default(),
            unit_price: Currency::new_float(0f64, None),
        }
    }

    pub fn id(&self) -> String {
        match &self.id {
            Some(id) => id.clone(),
            None => String::from("new"),
        }
    }

    pub fn dom_id(&self) -> String {
        match &self.id {
            Some(id) => format!("line_item_{}", id),
            None => format!("line_item_date_{}_line_item_new", &self.line_item_date_id),
        }
    }
}

impl From<LineItem> for LineItemPresenter {
    fn from(value: LineItem) -> Self {
        LineItemPresenter {
            id: Some(value.id),
            line_item_date_id: value.line_item_date_id,
            name: value.name,
            description: value.description.unwrap_or(String::from("")),
            quantity: value.quantity.to_string(),
            unit_price: value.unit_price,
        }
    }
}

impl From<LineItemForm> for LineItemPresenter {
    fn from(value: LineItemForm) -> Self {
        let unit_price = Currency::new_string(value.unit_price.as_str(), None)
            .unwrap_or(Currency::new_float(0f64, None));
        LineItemPresenter {
            id: value.id,
            line_item_date_id: value.line_item_date_id,
            name: value.name,
            description: value.description.unwrap_or(String::from("")),
            quantity: value.quantity.to_string(),
            unit_price,
        }
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct DeleteForm {
    pub(crate) id: String,
}
