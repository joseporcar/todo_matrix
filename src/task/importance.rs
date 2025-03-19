use std::fmt::Display;

use rusqlite::{types::{FromSql, ToSqlOutput}, ToSql};

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub enum Importance {
    Low,
    MidLow,
    Mid,
    MidHigh,
    High,
}
impl From<&[u8]> for Importance {
    fn from(value: &[u8]) -> Self {
        match value {
            b"Low" => Importance::Low,
            b"MidLow" => Importance::MidLow,
            b"Mid" => Importance::Mid,
            b"MidHigh" => Importance::MidHigh,
            b"High" => Importance::High,
            _ => Importance::Low,
        }
    }
}
impl Display for Importance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ToSql for Importance {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.to_string()))
    }
}

impl FromSql for Importance {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        match value {
            rusqlite::types::ValueRef::Text(items) => Ok(items.into()),
            _ => Err(rusqlite::types::FromSqlError::InvalidType)
        }
    }
}
