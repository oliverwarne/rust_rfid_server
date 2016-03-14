use rusqlite::Connection;
use std::path::Path;

use string;

struct Scanner {
    id: i32,
    name: String,
    status: String,
    open: bool,
    access_by: Vec<u8>,
}

fn sql_stuff() {
    let db_path = Path::new("home/oliver/prog/consult/scanning/scan_serv/test.db");
    let conn = Connection::open(&db_path).unwrap();
    
    conn.execute("CREATE TABLE scanners (
                  id                INTEGER PRIMARY KEY AUTOINCREMENT,
                  name              TEXT NOT NULL,
                  status            TEXT NOT NULL,
                  open              INTEGER NOT NULL,
                  access_by         BLOB NOT NULL
                  )", &[]).unwrap(); // Cmon man... why cant rust have optional args
    let test_scan = Scanner {
        id: 0,
        name: "Front door".to_string(),
        status: "Working normally".to_string(),
        open: false,
        access_by: string::prepare_string_vec(&vec!("hafd","hudf")),
    };

    conn.execute("INSERT INTO scanners (name, status, open, access_by)
                  VALUES ($1, $2, $3, $4)",
                 &[&test_scan.name, &test_scan.status, 
                 &test_scan.open, &test_scan.access_by]).unwrap();
}

pub fn main() {
    sql_stuff();
}

