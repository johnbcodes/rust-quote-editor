use crate::{currency::FORM_CURRENCY_REGEX, time::DATE_REGEX};
use once_cell::sync::Lazy;
use regex::Regex;
use rocket::form::{Contextual, Form};

pub(crate) static QUANTITY_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\d+$").unwrap());

pub(crate) fn css_for_field<'b, T>(
    form: &Form<Contextual<'_, T>>,
    field: &'b str,
    default_class: &'b str,
    error_class: &'b str,
) -> String {
    if form.context.exact_field_errors(field).count() == 0 {
        default_class.to_string()
    } else {
        format!("{} {}", default_class, error_class)
    }
}

pub(crate) fn validate_date<'v>(date: &str) -> rocket::form::Result<'v, ()> {
    if date.is_empty() {
        Err(rocket::form::Error::validation("Please enter a date"))?;
    }
    if !DATE_REGEX.is_match(date) {
        Err(rocket::form::Error::validation("Please enter a valid date"))?;
    }

    Ok(())
}

pub(crate) fn validate_amount<'v>(amount: &str) -> rocket::form::Result<'v, ()> {
    if amount.is_empty() {
        Err(rocket::form::Error::validation("Please enter an amount"))?;
    }
    if !FORM_CURRENCY_REGEX.is_match(amount) {
        Err(rocket::form::Error::validation(
            "Please enter a valid amount",
        ))?;
    }

    Ok(())
}

pub(crate) fn validate_quantity<'v>(quantity: &str) -> rocket::form::Result<'v, ()> {
    if quantity.is_empty() {
        Err(rocket::form::Error::validation("Please enter a quantity"))?;
    }
    if !QUANTITY_REGEX.is_match(quantity) {
        Err(rocket::form::Error::validation(
            "Please enter a valid quantity",
        ))?;
    }

    Ok(())
}
