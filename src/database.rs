use crate::task_matrix::Task;
use rusqlite::{Connection, Result};

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
                dates       TEXT NOT NULL,
                content     TEXT NOT NULL,
                complete    INTEGER NOT NULL,
                importance  INTEGER NOT NULL,
                urgency     INTEGER NOT NULL
            );
        ", []).expect("Error at creating table");
    }
    pub fn add_task(&self, task: Task) -> Result<usize>{

        self.connection.execute("
            INSERT INTO task (dates, content, complete, ignorance, urgency) VALUES $1 $2 $3 $4 $5
        ", (
            task.dates().iter().map(|x| x.to_string()).collect::<Vec<String>>().join("_"),
            task.content(),
            1u8,
            2u8,
            3u8
        ))
    }

}




