use crate::task_matrix::{Completeness, Importance, task};
use chrono::NaiveDate;
use gtk::builders::ComboBoxBuilder;
use rusqlite::{fallible_iterator::FallibleIterator, types::FromSql, Connection, Result, ToSql};

pub struct Table {
    connection: Connection,
}
impl Table {
    pub fn at_memory() -> Table {
        // TODO LINK TO ACTUAL FILE NOT JUST MEM
        Table {
            connection: Connection::open_in_memory().expect("Failed to open database"),
        }
    }

    pub fn at_storage(path: &str) -> Table {
        Table { connection: Connection::open(path).expect("failed to open database") }
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
    pub fn add_task(&self, task: task) -> Result<usize> {
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
    fn ugly_dates_sql_workaround(dates: &Vec<NaiveDate>) -> String {
        dates
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("_")
    }

    pub fn get_all_tasks(&self) -> Vec<task> {
        let mut statement = self.connection.prepare("SELECT * from task").unwrap();
        statement.query([]).unwrap().map(|row| row.get::<_, Completeness>(0)).unwrap().collect()


    }
    pub fn get_completeness(&self) -> Vec<crate::task_matrix::Completeness> {
        let mut statement = self.connection.prepare("SELECT complete from task").unwrap();
        statement.query([]).unwrap().map(|row| row.get::<_, Completeness>(0)).unwrap().collect()
    }

    pub fn clear_table(&self) {
        self.connection.execute("DROP TABLE task", []).unwrap();
    }
}
