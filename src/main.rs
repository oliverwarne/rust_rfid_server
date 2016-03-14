extern crate hyper;
extern crate rusqlite;

mod string;
mod server;
mod sql;
mod key_storage;

fn main() {
    string::main();
    //sql::main();
    key_storage::main();
    server::main();
    println!("exec");
}
