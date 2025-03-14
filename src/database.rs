use crate::task_matrix::{Importance, Task};
use chrono::NaiveDate;
use rusqlite::{types::FromSql, Connection, Result, ToSql};

pub struct Table {
    connection: Connection
}
impl Table {
    pub fn at_memory() -> Table {
        // TODO LINK TO ACTUAL FILE NOT JUST MEM
        Table {connection: Connection::open_in_memory().expect("Failed to open database")}
    }

    pub fn at_storage(path:i32) -> Table {
        todo!()
    }
    pub fn create_table(&self) {
        // Dates separated by an _
        self.connection.execute("
            CREATE TABLE IF NOT EXISTS task (
                id          INTEGER PRIMARY KEY,
                dates       BLOB NOT NULL,
                content     TEXT NOT NULL,
                complete    BLOB NOT NULL,
                importance  BLOB NOT NULL,
                urgency     BLOB NOT NULL
            );
        ", []).expect("Error at creating table");
    }
    pub fn add_task(&self, task: Task) -> Result<usize>{

        self.connection.execute("
            INSERT INTO task (dates, content, complete, ignorance, urgency) VALUES $1 $2 $3 $4 $5
        ", (
            Table::ugly_dates_sql_workaround(task.dates()),
            task.content(),
            task.completeness(),
            2u8,
            3u8
        ))
    }
    fn ugly_dates_sql_workaround(dates: &Vec<NaiveDate>) -> String {
        dates.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("_")
    }

    pub fn get_all(&self) -> Vec<Task> {
        let mut statement = self.connection.prepare("SELECT * from T").unwrap();
        // statement.query_map([], |row|
        //     todo!()
        // ).iter();//.collect();
        todo!()
    }

}


