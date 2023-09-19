use std::{
  collections::HashMap,
  io::Write,
  net::{TcpListener, TcpStream, ToSocketAddrs},
  thread,
};

use termstorage_protocol::{response::Response, Fetch, Protocol};
use termstorage_term::Term;

pub struct Server {
  listener: TcpListener,
  storage: HashMap<String, Term>,
}

impl Server {
  pub fn new<A: ToSocketAddrs>(addr: A) -> Self {
    let listener = TcpListener::bind(addr).unwrap();
    let storage = Default::default();

    Self { listener, storage }
  }
}

impl Server {
  pub fn start(self) -> thread::JoinHandle<()> {
    thread::spawn(move || loop {
      for incoming in self.listener.incoming() {
        match incoming {
          Ok(mut stream) => {
            let req = termstorage_protocol::decode(&mut stream);

            match req {
              Ok(prot) => self.handle(&mut stream, prot).unwrap(),
              Err(_) => println!("something bad occurred"),
            };
          }
          Err(_) => {}
        }
      }
    })
  }

  fn handle(
    &self,
    stream: &mut TcpStream,
    prot: Protocol,
  ) -> std::io::Result<()> {
    let resp = match prot {
      Protocol::Fetch(fetch) => self.handle_fetch(fetch),
      Protocol::Set(_) => todo!(),
      Protocol::Delete(_) => todo!(),
    };

    let encoded_resp = termstorage_protocol::response::encode(resp)?;
    stream.write(&encoded_resp)?;

    Ok(())
  }

  fn handle_fetch(&self, Fetch { name }: Fetch) -> Response {
    let term_opt = self.storage.get(&name).cloned();
    match term_opt {
      Some(term) => Response::Ok(term),
      None => Response::NotFound,
    }
  }
}
