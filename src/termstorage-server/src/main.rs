use std::thread;

use termstorage_client::Client;
use termstorage_protocol::{Delete, Fetch, Set};
use termstorage_server::Server;
use termstorage_term::Term;

fn main() {
  println!("Hello, world!");

  let server = Server::new("127.0.0.1:8080");

  let server_handle = server.start();

  let client = Client::new("127.0.0.1:8080");

  let req_handle = thread::spawn(move || {
    let resp = client.send(
      Set {
        name: "batata".to_string(),
        payload: Term::String("termstorage works".to_string()),
      }
      .into(),
    );
    println!("{resp:?}");

    let resp = client.send(
      Fetch {
        name: "batata".to_string(),
      }
      .into(),
    );
    println!("{resp:?}");

    let resp = client.send(
      Delete {
        name: "batata".to_string(),
      }
      .into(),
    );
    println!("{resp:?}");

    let resp = client.send(
      Fetch {
        name: "batata".to_string(),
      }
      .into(),
    );
    println!("{resp:?}");
  });

  server_handle.join().unwrap();
  req_handle.join().unwrap();
}
