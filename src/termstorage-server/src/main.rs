use std::thread;

use termstorage_client::Client;
use termstorage_protocol::{Delete, Fetch, Protocol, Set};
use termstorage_server::Server;
use termstorage_term::Term;

fn main() {
  println!("Hello, world!");

  let server = Server::new("127.0.0.1:8080");

  let server_handle = server.start();

  let client = Client::new("127.0.0.1:8080");

  let req_handle = thread::spawn(move || {
    let req = Protocol::Set(Set {
      name: "batata".to_string(),
      payload: Term::String("termstorage works".to_string()),
    });

    let resp = client.send(req);
    println!("{resp:?}");

    let req = Protocol::Fetch(Fetch {
      name: "batata".to_string(),
    });

    let resp = client.send(req);
    println!("{resp:?}");

    let req = Protocol::Delete(Delete {
      name: "batata".to_string(),
    });

    let resp = client.send(req);
    println!("{resp:?}");

    let req = Protocol::Fetch(Fetch {
      name: "batata".to_string(),
    });

    let resp = client.send(req);
    println!("{resp:?}");
  });

  server_handle.join().unwrap();
  req_handle.join().unwrap();
}
