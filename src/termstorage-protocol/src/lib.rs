pub mod response;

use std::io::{self, Read, Result, Write};

use termstorage_encoding::{Decode, Encode};
use termstorage_term::{self, Term};

use byteorder::{NetworkEndian, ReadBytesExt, WriteBytesExt};

#[derive(Debug)]
pub struct Fetch {
  pub name: String,
}

#[derive(Debug)]
pub struct Set {
  pub name: String,
  pub payload: Term,
}

#[derive(Debug)]
pub struct Delete {
  pub name: String,
}

#[derive(Debug)]
pub enum Protocol {
  Fetch(Fetch),
  Set(Set),
  Delete(Delete),
}

impl Encode for Protocol {
  fn encode(&self) -> std::io::Result<Vec<u8>> {
    encode(self)
  }
}

impl Decode for Protocol {
  fn decode(rd: &mut impl Read) -> std::io::Result<Self> {
    decode(rd)
  }
}

const TAG_FETCH: u8 = 10;
const TAG_SET: u8 = 11;
const TAG_DELETE: u8 = 12;

/// Encodes the given Protocol.
fn encode(prot: &Protocol) -> Result<Vec<u8>> {
  let mut buf = Vec::new();

  match prot {
    Protocol::Fetch(Fetch { name }) => {
      buf.write_u8(TAG_FETCH)?;
      write_name(&name, &mut buf)?;
    }
    Protocol::Set(Set { name, payload }) => {
      buf.write_u8(TAG_SET)?;
      write_name(&name, &mut buf)?;

      let payload = payload.encode()?;
      let payload_size = payload.len().to_be_bytes();

      buf.write(&payload_size)?;
      buf.write_all(&payload)?;
    }
    Protocol::Delete(Delete { name }) => {
      buf.write_u8(TAG_DELETE)?;
      write_name(&name, &mut buf)?;
    }
  }

  Ok(buf)
}

/// Decodes a Protocol with the given reader.
fn decode(reader: &mut impl Read) -> Result<Protocol> {
  let tag = reader.read_u8()?;

  match tag {
    TAG_FETCH => {
      let name = read_name(reader)?;

      let prot = Protocol::Fetch(Fetch { name });
      Ok(prot)
    }
    TAG_SET => {
      let name = read_name(reader)?;

      let payload_size = reader.read_uint::<NetworkEndian>(8)?;

      let mut buf_payload = vec![0u8; payload_size as usize];
      reader.read_exact(&mut buf_payload)?;

      let payload = Term::decode(&mut buf_payload.as_slice())?;

      let prot = Set { name, payload };
      Ok(Protocol::Set(prot))
    }
    TAG_DELETE => {
      let name = read_name(reader)?;
      let prot = Protocol::Delete(Delete { name });
      Ok(prot)
    }
    _ => {
      let err = io::Error::new(io::ErrorKind::InvalidData, "Invalid tag");
      Err(err)
    }
  }
}

fn write_name(name: &str, writer: &mut impl Write) -> Result<()> {
  let name_len = name.len();
  writer.write_uint::<NetworkEndian>(name_len as u64, 8)?;

  let name = name.as_bytes();
  writer.write_all(name)?;

  Ok(())
}

fn read_name(reader: &mut impl Read) -> Result<String> {
  let name_size = reader.read_uint::<NetworkEndian>(8)?;

  let mut name_buf = vec![0u8; name_size as usize];
  reader.read_exact(&mut name_buf)?;

  let name = String::from_utf8(name_buf);

  name.or_else(|e| {
    let err = io::Error::new(io::ErrorKind::InvalidInput, e);
    Err(err)
  })
}

impl Into<Protocol> for Fetch {
  fn into(self) -> Protocol {
    Protocol::Fetch(self)
  }
}

impl Into<Protocol> for Set {
  fn into(self) -> Protocol {
    Protocol::Set(self)
  }
}

impl Into<Protocol> for Delete {
  fn into(self) -> Protocol {
    Protocol::Delete(self)
  }
}
