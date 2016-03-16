extern crate hyper;
extern crate rusqlite;

mod string;
mod server;
mod sql;
mod key_storage;

fn main() {
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
