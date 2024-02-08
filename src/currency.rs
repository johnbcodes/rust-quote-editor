use once_cell::sync::Lazy;
use regex::Regex;

pub(crate) static FORM_CURRENCY_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^\d+(\.\d{2})?$").unwrap());
