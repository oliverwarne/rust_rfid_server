use rusqlite::Connection;
use std::path::Path;

use string;

#[derive(Debug)]
struct Scanner {
    id: i32,
    name: String,
    status: String,
    open: bool,
    access_by: Vec<u8>,
}

pub fn get_connection() -> Connection {
    let db_path = Path::new("./test.sqlite3");
    let conn = Connection::open(&db_path).unwrap();
    return conn;
}

pub fn create_table() {
    let conn = get_connection();
    conn.execute("CREATE TABLE scanners (
                  id                INTEGER PRIMARY KEY AUTOINCREMENT,
                  name              TEXT NOT NULL,
                  status            TEXT NOT NULL,
                  open              INTEGER NOT NULL,
                  access_by         BLOB NOT NULL
                  )", &[]).unwrap(); // Cmon man... why cant rust have optional args
}

pub fn get_access_bytearray(conn: &Connection, id: &i32) -> Vec<u8> {
    conn.query_row("SELECT access_by FROM scanners WHERE id=$1", &[id], |row| {
                    row.get(0)
    }).unwrap()
}

pub fn print_all_scanner(conn: &Connection) {
    let mut stmt = conn.prepare("SELECT id, name, status, open, access_by FROM scanners").unwrap();
    let scanners = stmt.query_map(&[], |row| {
        Scanner {
            id: row.get(0),
            name: row.get(1),
            status: row.get(2),
            open: row.get(3),
            access_by: row.get(4)
        }
    }).unwrap();
    
    for sql_scanner in scanners {
        let scanner: Scanner = sql_scanner.unwrap();
        print!("Found scanner #{:?}\n   Openable by: ", scanner.id);
        print!("{:?}\n", string::read_output_blob(&scanner.access_by))
    }
}

fn test_insert() {
    let conn = get_connection();
    
    let test_scan = Scanner {
        id: 0,
        name: "Front door".to_string(),
        status: "Working normally".to_string(),
        open: false,
        access_by: string::prepare_str_vec(&vec!("hafd","hudf")),
    };

    conn.execute("INSERT INTO scanners (name, status, open, access_by)
                  VALUES ($1, $2, $3, $4)",
                 &[&test_scan.name, &test_scan.status, 
                 &test_scan.open, &test_scan.access_by]).unwrap();
    print_all_scanner(&conn);
}

pub fn main() {
    test_insert();
}

