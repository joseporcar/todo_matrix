use std::fmt::Display;

use rusqlite::{
    ToSql,
    types::{FromSql, ToSqlOutput},
};

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub enum Completeness {
    None,
    Started,
    Almost,
    Complete,
}

impl From<&[u8]> for Completeness {
    fn from(value: &[u8]) -> Self {
        match value[0] {
            b'N' => Completeness::None,
            b'S' => Completeness::Started,
            b'A' => Completeness::Almost,
            b'C' => Completeness::Complete,
            _ => Completeness::None,
        }
    }
}

impl Display for Completeness {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ToSql for Completeness {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.to_string()))
    }
}

impl FromSql for Completeness {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        match value {
            rusqlite::types::ValueRef::Text(items) => Ok(items.into()),
            _ => Err(rusqlite::types::FromSqlError::InvalidType),
        }
    }
}
