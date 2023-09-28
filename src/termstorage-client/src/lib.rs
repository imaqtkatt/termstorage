use std::{
  io::{Result, Write},
  net::{TcpStream, ToSocketAddrs},
};

use termstorage_encoding::{Decode, Encode};
use termstorage_protocol::{response::Response, Protocol};

pub struct Client<Addr: ToSocketAddrs>(Addr);

impl<Addr: ToSocketAddrs> Client<Addr> {
  pub fn new(addr: Addr) -> Self {
    Self(addr)
  }

  pub fn send(&self, prot: Protocol) -> Result<Response> {
    let mut stream = TcpStream::connect(&self.0)?;

    let req = prot.encode()?;
    stream.write_all(&req)?;

    let resp = Response::decode(&mut stream)?;

    std::mem::drop(stream);

    Ok(resp)
  }
}
