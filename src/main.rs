extern crate hyper;
extern crate rusqlite;
extern crate rustc_serialize;

//mod string;
mod server;
//mod sql;
//mod key_storage;

fn main() {
    //string::main();
    //sql::main();
    server::main();
    println!("exec");
}
