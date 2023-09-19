use std::{
  io::Read,
  net::{TcpListener, ToSocketAddrs},
  thread,
};

pub struct Server {
  listener: TcpListener,
}

impl Server {
  pub fn new<A: ToSocketAddrs>(addr: A) -> Self {
    let listener = TcpListener::bind(addr).unwrap();
    Self { listener }
  }
}

impl Server {
  pub fn start(self) -> thread::JoinHandle<()> {
    thread::spawn(move || loop {
      for incoming in self.listener.incoming() {
        match incoming {
          Ok(mut stream) => {
            let mut buf = Vec::new();
            stream.read_to_end(&mut buf).unwrap();

            let data = termstorage_protocol::decode(&buf);

            match data {
              Ok(prot) => println!("PROT: {prot:?}"),
              Err(_) => println!("something bad occurred"),
            };
          }
          Err(_) => {}
        }
      }
    })
  }
}
