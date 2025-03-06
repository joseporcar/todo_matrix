use rusqlite::{Connection, Result};

pub fn open_conection() -> Connection {
    // TODO LINK TO ACTUAL FILE NOT JUST MEM
    Connection::open_in_memory().expect("Failed to open database")
}