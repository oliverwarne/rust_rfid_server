extern crate hyper;
extern crate rusqlite;

mod string;
mod server;
mod sql;
mod key_storage;

use key_storage::{KeyStore,KeyStoreResult};
use rusqlite::Connection;
use hyper::server::{Server,Response,Request};

pub fn has_access(scanner_id: &i32, input: &String, conn: &Connection) -> KeyStoreResult {
    let key_set: KeyStore = KeyStore::from_bytes(sql::get_occupied_bytearray(conn,scanner_id));
    return key_set.contains(input);
}

fn get_KeyStore(scanner_id: &i32) -> KeyStore {
    KeyStore::from_bytes(sql::get_occupied_bytearray(&sql::get_connection()
                                                     ,scanner_id))
}

fn runthru() {
    println!("str");
    string::main();
    println!("sql");
    sql::main();
    println!("key");
    key_storage::main();
    println!("main");
    server::main();
    println!("exec");
}

fn test_access() {
    let conn = sql::get_connection();
    sql::print_all_scanner(&conn);
    let allowed = has_access(&1, &"fuf".to_string(), &conn);
    println!("{:?}", allowed);
    conn.close();
}
/*
fn server() {
    Server::http("127.0.0.1:23").unwrap()
                                .handle().unwrap();
}
*/


fn main() {
    //sql::create_table()
    runthru();
    let keys1 = get_KeyStore(&1);
    println!("{:?}",keys1.contains("test"));
    let keys2 = get_KeyStore(&2);
    let keys3 = get_KeyStore(&3);
}
