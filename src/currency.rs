use currency_rs::Currency;
use once_cell::sync::Lazy;
use regex::Regex;
use rusqlite::{
    types::{FromSql, FromSqlError, FromSqlResult, ToSql, ToSqlOutput, ValueRef},
    Result,
};

pub(crate) static FORM_CURRENCY_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^\d*(\.\d{2})?$").unwrap());

pub(crate) struct CurrencySql(pub(crate) Currency);

impl FromSql for CurrencySql {
    fn column_result(value: ValueRef) -> FromSqlResult<Self> {
        match value {
            ValueRef::Integer(x) => Ok(CurrencySql(Currency::new_float(x as f64, None))),
            ValueRef::Real(x) => Ok(CurrencySql(Currency::new_float(x, None))),
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

impl ToSql for CurrencySql {
    fn to_sql(&self) -> Result<ToSqlOutput> {
        Ok(ToSqlOutput::from(self.0.value()))
    }
}
