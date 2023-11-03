use once_cell::sync::Lazy;
use regex::Regex;
use time::{format_description::FormatItem, macros::format_description, Date};

pub(crate) static DATE_FORMAT: &[FormatItem<'_>] = format_description!("[year]-[month]-[day]");
pub(crate) static DATE_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"[0-9]{4}-[0-9]{2}-[0-9]{2}$").unwrap());

pub(crate) fn long_form(date: Date) -> String {
    let mut result = String::with_capacity(18);
    result.push_str(date.month().to_string().as_str());
    result.push(' ');
    let day = format!("{:0>#2}", date.day());
    result.push_str(day.as_str());
    result.push_str(", ");
    result.push_str(date.year().to_string().as_str());
    result
}

pub(crate) fn short_form(date: Date) -> String {
    date.format(&format_description!("[year]-[month]-[day]"))
        .unwrap()
}

pub(crate) fn parse_date<S: AsRef<str>>(text: S) -> Date {
    Date::parse(text.as_ref(), DATE_FORMAT).unwrap()
}

#[cfg(test)]
mod tests {
    use time::{Date, Month};

    #[test]
    fn long_form() {
        let date = Date::from_calendar_date(2023, Month::April, 2).unwrap();
        assert_eq!("April 02, 2023", super::long_form(date));
    }
}
