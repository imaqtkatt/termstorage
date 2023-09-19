use std::{io::Write, net::TcpStream, thread};

use termstorage_protocol::{Fetch, Protocol};
use termstorage_server::Server;

fn main() {
  println!("Hello, world!");

  let server = Server::new("127.0.0.1:8080");

  let server_handle = server.start();

  let req_handle = thread::spawn(move || {
    for _ in 0..10 {
      let req = Protocol::Fetch(Fetch {
        name: "batata".to_string(),
      });

      let req = termstorage_protocol::encode(req).unwrap();

      let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();

      stream.write(&req).unwrap();

      let resp = termstorage_protocol::response::decode(&mut stream).unwrap();

      println!("{resp:?}");
    }
  });

  server_handle.join().unwrap();
  req_handle.join().unwrap();
}
