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
    let key_set: KeyStore = KeyStore::from_bytes(sql::get_access_bytearray(conn,scanner_id));
    return key_set.contains(input);
}

fn access(id: &i32, key: &String) -> KeyStoreResult {
    has_access(id, key, &sql::get_connection())
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
    println!("{:?} {:?}",access(&2,&"hafd".to_string()),access(&2,&"hud".to_string()));
}
