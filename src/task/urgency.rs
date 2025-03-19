use std::fmt::Display;

use rusqlite::{types::{FromSql, ToSqlOutput}, ToSql};

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub enum Urgency {
    Low,
    MidLow,
    Mid,
    MidHigh,
    High,
}
impl From<&[u8]> for Urgency {
    fn from(value: &[u8]) -> Self {
        match value {
            b"Low" => Urgency::Low,
            b"MidLow" => Urgency::MidLow,
            b"Mid" => Urgency::Mid,
            b"MidHigh" => Urgency::MidHigh,
            b"High" => Urgency::High,
            _ => Urgency::Low,
        }
    }
}
impl Display for Urgency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ToSql for Urgency {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.to_string()))
    }
}

impl FromSql for Urgency {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        match value {
            rusqlite::types::ValueRef::Text(items) => Ok(items.into()),
            _ => Err(rusqlite::types::FromSqlError::InvalidType)
        }
    }
}
