mod database;
mod graphics;
pub mod task_matrix;

use std::str::FromStr;

use chrono::NaiveDate;
pub use graphics::run_app;

pub fn testing() {
    use task_matrix::*;
    let conn = database::Table::at_memory();
    let dates = NaiveDate::from_str("2024-02-23").unwrap();
    let Task = Task::new(
        "Do hw".to_string(),
        vec![dates],
        Importance::High,
        Urgency::Low,
    );

    conn.create_table();
    conn.add_task(Task).unwrap();
}
