use rusqlite::{Connection, Result};

pub struct Table {
    connection: Connection
}
impl Table {
    pub fn open_conection() -> Connection {
        // TODO LINK TO ACTUAL FILE NOT JUST MEM
        Connection::open_in_memory().expect("Failed to open database")
    }
    pub fn create_table(&self) {
        self.connection.execute_batch("
            CREATE TABLE task (
                date        TEXT;
                content     TEXT;
                complete    INTEGER;
                importance  INTEGER;
                urgency     INTEGER;
            )
        ").expect("Error at creating table");
    }
}


