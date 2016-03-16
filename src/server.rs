use std::io::Read;
use hyper::server::{Server, Request, Response};

use key_storage::key_store;

fn key_init() -> key_store {
    let data: Vec<String> = vec!["h".to_string(),"e".to_string(),"l".to_string()];
    let key_set: key_store = key_store::new_from_vec(data);
    return key_set;
}

fn hello(mut req: Request, res: Response) {
    
    let mut buf = String::new();

    assert!(0 <= req.read_to_string(&mut buf).unwrap());

    println!("{:?}",buf);
    
    if key_init().contains(buf) {
        res.send(b"Contains!\n").unwrap();
    } else {
        res.send(b"Doesn't contain\n").unwrap();
    }
}

pub fn main() {
    println!("started");
    Server::http("127.0.0.1:23").unwrap()
                                .handle(hello).unwrap();
    println!("fin");
}
