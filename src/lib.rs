mod database;
mod graphics;
mod task;
mod matrix;

use std::str::FromStr;

use chrono::{NaiveDate, Utc};
pub use graphics::run_app;
use task::{Completeness, Importance, Task, Urgency};

pub fn testing() {
    let conn = database::Table::at_storage("./db.db3");
    let dates = NaiveDate::from_str("2024-02-23").unwrap();
    let task = Task::new(
        "Do hw".to_string(),
        Completeness::Almost,
        vec![dates, Utc::now().date_naive()],
        Importance::High,
        Urgency::Low,
    );
    // conn.clear_table();
    conn.create_table();
    conn.add_task(task).unwrap();
    println!("{:?}",conn.get_tasks_from_day(Utc::now().date_naive()))
}
