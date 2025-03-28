pub mod completeness;
pub mod importance;
pub mod urgency;

use std::str::FromStr;

use chrono::NaiveDate;
pub use completeness::Completeness;
pub use importance::Importance;
use rusqlite::Row;
pub use urgency::Urgency;

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
        complete: Completeness,
        dates: Vec<NaiveDate>,
        importance: Importance,
        urgency: Urgency,
    ) -> Task {
        Task {
            id: Uploaded::NotUploaded,
            content,
            complete,
            dates,
            importance,
            urgency,
        }
    }
    pub fn from_sql(
        id: u32,
        dates: String,
        content: String,
        complete: Completeness,
        importance: Importance,
        urgency: Urgency,
    ) -> Task {
        let dates = dates
            .split("_")
            .map(|date| NaiveDate::from_str(date).unwrap())
            .collect();

        Task {
            id: Uploaded::Uploaded(id),
            content,
            dates,
            complete,
            importance,
            urgency,
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
    pub fn get_index_importance_urgency(&self) -> (i32, i32) {
        (Self::get_index_helper(&self.importance), Self::get_index_helper(&self.urgency))
    }
    fn get_index_helper(element: &impl ToString) -> i32 {
        match element.to_string().as_str() {
            "High" => 5,
            "MidHigh" => 4,
            "Mid" => 3,
            "MidLow" => 2,
            "Low" => 1,
            _ => -1,
        }
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

impl From<&Row<'_>> for Task {
    fn from(value: &Row) -> Self {
        Task {
            id: Uploaded::Uploaded(value.get(0).unwrap()),
            dates: ugly_sql_dates_workaround(value.get(1).unwrap()),
            content: value.get(2).unwrap(),
            complete: value.get(3).unwrap(),
            importance: value.get(4).unwrap(),
            urgency: value.get(5).unwrap(),
        }
    }
}

fn ugly_sql_dates_workaround(dates: String) -> Vec<NaiveDate> {
    dates
        .split('_')
        .map(|date| NaiveDate::from_str(date).unwrap())
        .collect()
}
#[derive(Debug, Clone)]
enum Uploaded {
    Uploaded(u32),
    NotUploaded,
}
