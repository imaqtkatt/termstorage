pub mod response;

use std::io::{self, BufWriter, Read, Write};

use std::io::Result;

use termstorage_term::{self, Term};

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

const TAG_FETCH: u8 = 10;
const TAG_SET: u8 = 11;
const TAG_DELETE: u8 = 12;

/// Encodes the given protocol.
pub fn encode(prot: Protocol) -> Result<Vec<u8>> {
  let mut buf = Vec::new();

  let mut writer = BufWriter::new(&mut buf);

  match prot {
    Protocol::Fetch(Fetch { name }) => {
      writer.write(&[TAG_FETCH])?;
      write_name(&name, &mut writer)?;
    }
    Protocol::Set(Set { name, payload }) => {
      writer.write(&[TAG_SET])?;
      write_name(&name, &mut writer)?;

      let payload = termstorage_term::encode(&payload)?;
      let payload_size = payload.len().to_be_bytes();

      writer.write(&payload_size)?;
      writer.write_all(&payload)?;
    }
    Protocol::Delete(Delete { name }) => {
      writer.write(&[TAG_DELETE])?;
      write_name(&name, &mut writer)?;
    }
  }

  let vec = writer.buffer().to_vec();
  Ok(vec)
}

/// Decodes a Protocol with the given reader.
pub fn decode(reader: &mut impl Read) -> Result<Protocol> {
  // let mut reader = BufReader::new(slice);

  let mut buf = [0u8; 1];
  reader.read_exact(&mut buf)?;

  match buf[0] {
    TAG_FETCH => {
      let name = read_name(reader)?;

      let prot = Protocol::Fetch(Fetch { name });
      Ok(prot)
    }
    TAG_SET => {
      let name = read_name(reader)?;

      let mut buf_payload_size = [0u8; 8];
      reader.read_exact(&mut buf_payload_size)?;

      let payload_size = usize::from_be_bytes(buf_payload_size);

      let mut buf_payload = vec![0u8; payload_size];
      reader.read_exact(&mut buf_payload)?;

      let payload = termstorage_term::decode(&buf_payload)?;

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
  let name_size = name.len().to_be_bytes();
  writer.write(&name_size)?;

  let name = name.as_bytes();
  writer.write(name)?;

  Ok(())
}

fn read_name(reader: &mut impl Read) -> Result<String> {
  let mut name_size_buf = [0u8; 8];
  reader.read_exact(&mut name_size_buf)?;

  let name_size = usize::from_be_bytes(name_size_buf);

  let mut name_buf = vec![0u8; name_size];
  reader.read_exact(&mut name_buf)?;

  let name = String::from_utf8(name_buf);

  name.or_else(|e| {
    let err = io::Error::new(io::ErrorKind::InvalidInput, e);
    Err(err)
  })
}
