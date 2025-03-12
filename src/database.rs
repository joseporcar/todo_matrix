use rusqlite::{Connection, Result};

use crate::task_matrix::Task;

pub struct Table {
    connection: Connection
}
impl Table {
    pub fn open_conection() -> Connection {
        // TODO LINK TO ACTUAL FILE NOT JUST MEM
        Connection::open_in_memory().expect("Failed to open database")
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

}


