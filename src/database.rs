use crate::{matrix::DayMatrix, task::Task};
use chrono::NaiveDate;
use rusqlite::{Connection, Result, fallible_iterator::FallibleIterator};

pub struct Table {
    connection: Connection,
}
impl Table {
    pub fn at_memory() -> Table {
        Table {
            connection: Connection::open_in_memory().expect("Failed to open database"),
        }
    }

    pub fn at_storage(path: &str) -> Table {
        Table {
            connection: Connection::open(path).expect("failed to open database"),
        }
    }
    pub fn create_table(&self) {
        // Dates separated by an _
        self.connection
            .execute(
                "
            CREATE TABLE IF NOT EXISTS task (
                id          INTEGER PRIMARY KEY,
                dates       TEXT NOT NULL,
                content     TEXT NOT NULL,
                complete    TEXT NOT NULL,
                importance  TEXT NOT NULL,
                urgency     TEXT NOT NULL
            );
        ",
                [],
            )
            .expect("Error at creating table");
    }
    pub fn add_task(&self, task: Task) -> Result<usize> {
        self.connection.execute(
            "INSERT INTO task (dates, content, complete, importance, urgency) VALUES ($1, $2, $3, $4, $5)",
            (
                Table::ugly_dates_sql_workaround(task.dates()),
                task.content(),
                task.completeness(),
                task.importance(),
                task.urgency(),
            ),
        )
    }
    fn ugly_dates_sql_workaround(dates: &[NaiveDate]) -> String {
        dates
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("_")
    }

    pub fn get_all_tasks(&self) -> Vec<Task> {
        let mut statement = self.connection.prepare("SELECT * from task").unwrap();
        statement
            .query([])
            .unwrap()
            .map(|row| Ok(Task::from(row)))
            .unwrap()
            .collect()
    }

    pub fn get_tasks_from_day(&self, date: NaiveDate) -> DayMatrix {
        let str_date = date.to_string();
        let mut statement = self
            .connection
            .prepare(&format!(
                "SELECT * from task WHERE dates LIKE '%{}%'",
                str_date
            ))
            .unwrap();
        let tasks = statement
            .query([])
            .unwrap()
            .map(|row| Ok(Task::from(row)))
            .unwrap()
            .collect();

        DayMatrix::new(date, tasks)
    }
    pub fn clear_table(&self) {
        self.connection.execute("DROP TABLE task", []).unwrap();
    }
}
