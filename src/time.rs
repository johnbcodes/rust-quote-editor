use once_cell::sync::Lazy;
use regex::Regex;
use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ToSql, ToSqlOutput, ValueRef};
use rusqlite::Result;
use time::{
    format_description::{well_known::Rfc3339, FormatItem},
    macros::format_description,
    Date, OffsetDateTime,
};

pub(crate) static DATE_FORMAT: &[FormatItem<'_>] = format_description!("[year]-[month]-[day]");
pub(crate) static DATE_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"[0-9]{4}-[0-9]{2}-[0-9]{2}$").unwrap());

pub(crate) struct DateTimeSql(pub(crate) OffsetDateTime);

impl FromSql for DateTimeSql {
    fn column_result(value: ValueRef) -> FromSqlResult<Self> {
        String::column_result(value).and_then(|as_string| {
            OffsetDateTime::parse(as_string.as_ref(), &Rfc3339)
                .map(DateTimeSql)
                .map_err(|err| FromSqlError::Other(Box::new(err)))
        })
    }
}

impl ToSql for DateTimeSql {
    //noinspection DuplicatedCode
    fn to_sql(&self) -> Result<ToSqlOutput> {
        let time_string = self
            .0
            .format(&Rfc3339)
            .map_err(|err| FromSqlError::Other(Box::new(err)))?;
        Ok(ToSqlOutput::from(time_string))
    }
}

pub(crate) struct DateSql(pub(crate) Date);

impl FromSql for DateSql {
    fn column_result(value: ValueRef) -> FromSqlResult<Self> {
        String::column_result(value).and_then(|as_string| {
            Date::parse(as_string.as_ref(), &DATE_FORMAT)
                .map(DateSql)
                .map_err(|err| FromSqlError::Other(Box::new(err)))
        })
    }
}

impl ToSql for DateSql {
    //noinspection DuplicatedCode
    fn to_sql(&self) -> Result<ToSqlOutput> {
        let date_string = self
            .0
            .format(&DATE_FORMAT)
            .map_err(|err| FromSqlError::Other(Box::new(err)))?;
        Ok(ToSqlOutput::from(date_string))
    }
}

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
