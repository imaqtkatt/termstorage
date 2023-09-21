use std::{
  io::{Result, Write},
  net::{TcpStream, ToSocketAddrs},
};

use termstorage_protocol::{response::Response, Protocol};

pub struct Client<Addr: ToSocketAddrs>(Addr);

impl<Addr: ToSocketAddrs> Client<Addr> {
  pub fn new(addr: Addr) -> Self {
    Self(addr)
  }

  pub fn send(&self, prot: Protocol) -> Result<Response> {
    let mut stream = TcpStream::connect(&self.0)?;

    let req = termstorage_protocol::encode(prot)?;
    stream.write_all(&req)?;

    let resp = termstorage_protocol::response::decode(&mut stream)?;

    std::mem::drop(stream);

    Ok(resp)
  }
}
