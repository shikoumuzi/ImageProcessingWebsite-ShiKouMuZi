use rusqlite;
use rusqlite::{Connection, Transaction};
use std::option::Option;
use std::string::ToString;

pub struct SQLite{
    db_path: String,
    pub con: rusqlite::Connection,
}

impl SQLite{
    pub fn new(db_path: &str) -> SQLite {
        let con = Connection::open(db_path);

        // assert!(con.is_err(), "{}", con.err().unwrap().to_string().as_str());

        return SQLite {
            db_path: db_path.to_string(),
            con: con.unwrap()
        }
    }

    pub fn getTransaction(&mut self) -> rusqlite::Result<Transaction<'_>> {
        return self.con.transaction();
    }

    pub fn getConn(&mut self) -> &Connection{
        return &self.con
    }
}
