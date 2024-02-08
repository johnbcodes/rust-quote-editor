use crate::{
    forms::{validate_amount, validate_quantity},
    schema::line_items,
};
use currency_rs::Currency;
use diesel::prelude::*;
use time::OffsetDateTime;
use ulid::Ulid;

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

// FIXME: Should be TryFrom due to potential bad parse from quantity
impl From<&EditLineItemForm> for LineItem {
    fn from(value: &EditLineItemForm) -> Self {
        let description = value.description.clone().unwrap_or(String::from(""));
        let description = if description.is_empty() {
            None
        } else {
            Some(description)
        };
        LineItem {
            id: value.id.clone(),
            line_item_date_id: value.line_item_date_id.clone(),
            name: value.name.clone(),
            description,
            quantity: value.quantity.parse::<i32>().unwrap_or(0),
            unit_price: Currency::new_string(value.unit_price.as_str(), None)
                .unwrap_or(Currency::new_float(0f64, None)),
            created_at: OffsetDateTime::now_utc(),
            updated_at: OffsetDateTime::now_utc(),
        }
    }
}

// FIXME: Should be TryFrom due to potential bad parse from quantity
impl From<&NewLineItemForm> for LineItem {
    fn from(value: &NewLineItemForm) -> Self {
        let description = value.description.clone().unwrap_or(String::from(""));
        let description = if description.is_empty() {
            None
        } else {
            Some(description)
        };
        LineItem {
            id: Ulid::new().to_string(),
            line_item_date_id: value.line_item_date_id.clone(),
            name: value.name.clone(),
            description,
            quantity: value.quantity.parse::<i32>().unwrap_or(0),
            unit_price: Currency::new_string(value.unit_price.as_str(), None)
                .unwrap_or(Currency::new_float(0f64, None)),
            created_at: OffsetDateTime::now_utc(),
            updated_at: OffsetDateTime::now_utc(),
        }
    }
}

#[derive(Clone, Debug, FromForm)]
pub struct EditLineItemForm {
    #[field(validate = len(1..))]
    pub(crate) id: String,
    #[field(validate = len(1..))]
    pub(crate) line_item_date_id: String,
    #[field(validate = len(1..))]
    pub(crate) quote_id: String,
    #[field(validate = len(1..).or_else(msg!("Please enter a name")))]
    pub(crate) name: String,
    pub(crate) description: Option<String>,
    #[field(validate = validate_quantity())]
    pub(crate) quantity: String,
    #[field(validate = validate_amount())]
    pub(crate) unit_price: String,
}

#[derive(Clone, Debug, FromForm)]
pub struct NewLineItemForm {
    #[field(validate = len(1..))]
    pub(crate) line_item_date_id: String,
    #[field(validate = len(1..))]
    pub(crate) quote_id: String,
    #[field(validate = len(1..).or_else(msg!("Please enter a name")))]
    pub(crate) name: String,
    pub(crate) description: Option<String>,
    #[field(validate = validate_quantity())]
    pub(crate) quantity: String,
    #[field(validate = validate_amount())]
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

impl From<EditLineItemForm> for LineItemPresenter {
    fn from(value: EditLineItemForm) -> Self {
        let unit_price = Currency::new_string(value.unit_price.as_str(), None)
            .unwrap_or(Currency::new_float(0f64, None));
        LineItemPresenter {
            id: Some(value.id),
            line_item_date_id: value.line_item_date_id,
            name: value.name,
            description: value.description.unwrap_or(String::from("")),
            quantity: value.quantity.to_string(),
            unit_price,
        }
    }
}

impl From<NewLineItemForm> for LineItemPresenter {
    fn from(value: NewLineItemForm) -> Self {
        let unit_price = Currency::new_string(value.unit_price.as_str(), None)
            .unwrap_or(Currency::new_float(0f64, None));
        LineItemPresenter {
            id: None,
            line_item_date_id: value.line_item_date_id,
            name: value.name,
            description: value.description.unwrap_or(String::from("")),
            quantity: value.quantity.to_string(),
            unit_price,
        }
    }
}

#[derive(Debug, FromForm)]
pub(crate) struct DeleteForm {
    pub(crate) id: String,
}
