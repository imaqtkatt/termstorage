use std::{
  collections::HashMap,
  io::Write,
  net::{TcpListener, TcpStream, ToSocketAddrs},
  thread,
};

use termstorage_encoding::{Decode, Encode};
use termstorage_protocol::{response::Response, Delete, Fetch, Protocol, Set};
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
  pub fn start(mut self) -> thread::JoinHandle<()> {
    thread::spawn(move || loop {
      let accept = self.listener.accept();
      match accept {
        Ok((mut stream, _)) => {
          let req = Protocol::decode(&mut stream);
          match req {
            Ok(prot) => self.handle(&mut stream, prot).unwrap(),
            Err(_) => self.unprocessed(&mut stream).unwrap(),
          };
        }
        Err(_) => {}
      }
    })
  }

  fn handle(
    &mut self,
    stream: &mut TcpStream,
    prot: Protocol,
  ) -> std::io::Result<()> {
    let resp = match prot {
      Protocol::Fetch(fetch) => self.handle_fetch(fetch),
      Protocol::Set(set) => self.handle_set(set),
      Protocol::Delete(delete) => self.handle_delete(delete),
    };

    let encoded_resp = resp.encode()?;
    stream.write(&encoded_resp)?;

    Ok(())
  }

  fn handle_fetch(&self, Fetch { ref name }: Fetch) -> Response {
    let term_opt = self.storage.get(name).cloned();
    match term_opt {
      Some(term) => Response::Ok(term),
      None => Response::NotFound,
    }
  }

  fn handle_set(&mut self, Set { name, payload }: Set) -> Response {
    self.storage.insert(name, payload);
    Response::Processed
  }

  fn handle_delete(&mut self, Delete { ref name }: Delete) -> Response {
    self.storage.remove(name);
    Response::Processed
  }

  fn unprocessed(&self, stream: &mut TcpStream) -> std::io::Result<()> {
    let res = Response::Unprocessed.encode()?;
    stream.write(&res)?;

    Ok(())
  }
}
