use std::{fmt::Display, str::FromStr};

use chrono::{NaiveDate, Utc};
use rusqlite::{types::{FromSql, ToSqlOutput}, ToSql};
pub struct DayMatrix {
    date: NaiveDate,
    tasks: Vec<Task>,
}
impl DayMatrix {
    pub fn default() -> DayMatrix {
        DayMatrix {
            date: Utc::now().date_naive(),
            tasks: Vec::new(),
        }
    }
    pub fn date(&self) -> &NaiveDate {
        &self.date
    }
    pub fn tasks(&self) -> &Vec<Task> {
        &self.tasks
    }
    pub fn task(&self, index: usize) -> &Task {
        &self.tasks[index]
    }
    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }
    pub fn sort_by_importance(&mut self) {
        self.tasks.sort_by(|a, b| a.compare_importance(b));
    }
    pub fn sort_by_urgency(&mut self) {
        self.tasks.sort_by(|a, b| a.compare_urgency(b));
    }
}

// impl FromIterator<Task> for Matrix {
//     fn from_iter<T: IntoIterator<Item = Task>>(iter: T) -> Self {
//         Matrix { dates:Utc::now().date_naive(), tasks: iter.into_iter().collect() }
//     }
// }

// impl From<&[Task]> for Matrix {
//     fn from(value: &[Task]) -> Self {
//         Matrix { date: Utc::now().date_naive(), tasks: value.into() }
//     }
// }

#[derive(Debug, Clone)]
pub struct Task {
    id: Uploaded,
    content: String,
    complete: Completeness,
    dates: Vec<NaiveDate>,
    importance: Importance,
    urgency: Urgency,
}

impl Task {
    pub fn new(
        content: String,
        dates: Vec<NaiveDate>,
        importance: Importance,
        urgency: Urgency,
    ) -> Task {
        Task {
            id: Uploaded::NotUploaded,
            content,
            complete: Completeness::None,
            dates,
            importance,
            urgency,
        }
    }
    pub fn from_sql(id: u32, dates: String, content: String, complete: Completeness, importance: Importance, urgency: Urgency) -> Task {
        let dates = dates.split("_").map(|date| NaiveDate::from_str(date).unwrap()).collect();

        Task {
            id: Uploaded::Uploaded(id),
            content,
            dates,
            complete,
            importance,
            urgency
        }
    }
    pub fn set_id(&mut self, id: u32) {
        self.id = Uploaded::Uploaded(id)
    }
    pub fn id(&self) -> Option<u32> {
        match self.id {
            Uploaded::Uploaded(id) => Some(id),
            Uploaded::NotUploaded => None,
        }
    }
    pub fn compare_importance(&self, other: &Task) -> std::cmp::Ordering {
        self.importance.cmp(&other.importance)
    }
    pub fn compare_urgency(&self, other: &Task) -> std::cmp::Ordering {
        self.urgency.cmp(&other.urgency)
    }
    pub fn content(&self) -> &str {
        &self.content
    }
    pub fn completeness(&self) -> &Completeness {
        &self.complete
    }
    pub fn set_completeness(&mut self, completeness: Completeness) {
        self.complete = completeness
    }
    pub fn dates(&self) -> &Vec<NaiveDate> {
        &self.dates
    }
    pub fn urgency(&self) -> &Urgency {
        &self.urgency
    }
    pub fn importance(&self) -> &Importance {
        &self.importance
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub enum Importance {
    Low,
    MidLow,
    Mid,
    MidHigh,
    High,
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

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub enum Urgency {
    Low,
    MidLow,
    Mid,
    MidHigh,
    High,
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

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub enum Completeness {
    None,
    Started,
    Almost,
    Complete,
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
        todo!()
    }
}

#[derive(Debug, Clone)]
enum Uploaded {
    Uploaded(u32),
    NotUploaded,
}
