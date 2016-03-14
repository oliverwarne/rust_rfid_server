use std::io::Read;
use hyper::server::{Server, Request, Response};


fn hello(mut req: Request, res: Response) {
    
    let mut buf = String::new();

    println!("{:?}",req.read_to_string(&mut buf));

    println!("{:?}",buf);

    println!("{:?}",req.headers);

    res.send(b"Hello World!\n").unwrap();
}

pub fn main() {
        Server::http("127.0.0.1:23").unwrap()
                    .handle(hello).unwrap();
}
