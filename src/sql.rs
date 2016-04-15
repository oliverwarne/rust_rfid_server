use rusqlite::{Connection,Error};
use std::path::Path;

use string;

#[derive(Debug)]
struct Scanner {
    id: i32,
    name: String,
    status: String,
    avaliable: bool,
    occupied: Vec<u8>,
}

pub fn get_connection() -> Connection {
    let db_path = Path::new("./scanner.db");
    let conn = Connection::open(&db_path).unwrap();
    return conn;
}

pub fn create_table() {
    let conn = get_connection();
    conn.execute("CREATE TABLE scanners (
                  id                INTEGER PRIMARY KEY AUTOINCREMENT,
                  name              TEXT NOT NULL,
                  status            TEXT NOT NULL,
                  avaliable         INTEGER NOT NULL,
                  occupied          BLOB
                  )", &[]).unwrap(); // Cmon man... why cant rust have optional args
}

fn fill_table() {
    let conn = get_connection();
    let statement_array = [
        "INSERT INTO scanners (name, status, avaliable, blob) 
         VALUES ('Front', 'Open', true,  None)",
        "INSERT INTO scanners (name, status, avaliable, blob) 
         VALUES ('Left',   'Open', false, None)",
        "INSERT INTO scanners (name, status, avaliable, blob) 
         VALUES ('Gross',  'Open', true,  None)",
        "INSERT INTO scanners (name, status, avaliable, blob) 
         VALUES ('Apple',  'Open', true,  None)",
        "INSERT INTO scanners (name, status, avaliable, blob) 
         VALUES ('Yanqui', 'Open', false, None)",
        "INSERT INTO scanners (name, status, avaliable, blob) 
         VALUES ('Fists',  'Open', true,  None)",
    ];
    for statement in statement_array.iter() {
        conn.execute(statement,&[]).unwrap();
    }
    println!("Table has been filled!");
}

pub fn get_occupied_bytearray(conn: &Connection, id: &i32) -> Vec<u8> {
    match conn.query_row("SELECT occupied FROM scanners WHERE id=$1", &[id], |row| {
                    row.get(0)
    }) {
        Err(why) => match why {
                    // SQLite doesn't like returning blanks, so it throws an error
                    Error::QueryReturnedNoRows => return vec![], 
                    
                    Error::SqliteFailure(err,err_string) => match err_string.unwrap().as_ref()
                    {
                        "no such table: scanners"   => 
                        panic!("Scanner table doesn't exist! run create_table()"),

                        // Big ole' OR statement for handling malformed database
                        "no such column: id"        |
                        "no such column: name"      |
                        "no such column: status"    |
                        "no such column: avaliable" |
                        "no such column: occupied"  =>
                            panic!("Malformed database! not all columns have been createdtry running create_table()"),

                    _ => panic!("SQLITE FAILURE IN RETREIVING BYTEARRAY"),
                    },

                    _ => { println!("{:?}", &why);
                            panic!(why);
                    }
        },
        Ok(val)  => val,
    }
}

pub fn print_all_scanner(conn: &Connection) {
    let mut stmt = conn.prepare("SELECT id, name, status, avaliable, occupied 
                                 FROM scanners").unwrap();
    let scanners = stmt.query_map(&[], |row| {
        Scanner {
            id: row.get(0),
            name: row.get(1),
            status: row.get(2),
            avaliable: row.get(3),
            occupied: row.get(4)
        }
    }).unwrap();
    
    for sql_scanner in scanners {
        let scanner: Scanner = sql_scanner.unwrap();
        print!("Found scanner #{:?}\n   Openable by: ", scanner.id);

        for i in string::read_output_blob(&scanner.occupied) {
            print!("{:?}\n",i);
        }
    }
}

fn test_insert() {
    let conn = get_connection();
    
    let test_scan = Scanner {
        id: 0,
        name: "Front door".to_string(),
        status: "Working normally".to_string(),
        avaliable: false,
        occupied: string::prepare_str_vec(&vec!("hafd","hudf")),
    };

    conn.execute("INSERT INTO scanners (name, status, avaliable, occupied)
                  VALUES ($1, $2, $3, $4)",
                 &[&test_scan.name, &test_scan.status, 
                 &test_scan.avaliable, &test_scan.occupied]).unwrap();

    print_all_scanner(&conn);
    
    conn.close().unwrap();
}

pub fn main() {
    print_all_scanner(&get_connection());
}


