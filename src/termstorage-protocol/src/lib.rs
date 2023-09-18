use std::io::{BufReader, BufWriter, Read, Write};

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

pub fn encode(prot: Protocol) -> Vec<u8> {
  let mut encd = Vec::new();

  let mut writer = BufWriter::new(&mut encd);

  match prot {
    Protocol::Fetch(Fetch { name }) => {
      writer.write(&[TAG_FETCH]).unwrap();
      write_name(&name, &mut writer);
    }
    Protocol::Set(Set { name, payload }) => {
      writer.write(&[TAG_SET]).unwrap();
      write_name(&name, &mut writer);

      let payload = termstorage_term::encode(&payload);
      let payload_size = payload.len().to_be_bytes();

      writer.write(&payload_size).unwrap();
      writer.write_all(&payload).unwrap();
    }
    Protocol::Delete(Delete { name }) => {
      writer.write(&[TAG_DELETE]).unwrap();
      write_name(&name, &mut writer);
    }
  }

  writer.buffer().to_vec()
}

pub fn decode(slice: &[u8]) -> Option<Protocol> {
  let mut reader = BufReader::new(slice);

  let mut buf = [0u8; 1];
  reader.read_exact(&mut buf).unwrap();

  match buf[0] {
    TAG_FETCH => {
      let name = read_name(&mut reader)?;

      let prot = Protocol::Fetch(Fetch { name });
      Some(prot)
    }
    TAG_SET => {
      let name = read_name(&mut reader)?;

      let mut buf_payload_size = [0u8; 8];
      reader.read_exact(&mut buf_payload_size).unwrap();

      let payload_size = usize::from_be_bytes(buf_payload_size);

      let mut buf_payload = vec![0u8; payload_size];
      reader.read_exact(&mut buf_payload).unwrap();

      let payload = termstorage_term::decode(&buf_payload)?;

      let prot = Set { name, payload };
      Some(Protocol::Set(prot))
    }
    TAG_DELETE => {
      let name = read_name(&mut reader)?;
      let prot = Protocol::Delete(Delete { name });
      Some(prot)
    }
    _ => return None,
  }
}

fn write_name(name: &str, writer: &mut impl Write) {
  let name_size = name.len().to_be_bytes();
  writer.write(&name_size).unwrap();

  let name = name.as_bytes();
  writer.write(name).unwrap();
}

fn read_name(reader: &mut impl Read) -> Option<String> {
  let mut name_size_buf = [0u8; 8];
  reader.read_exact(&mut name_size_buf).unwrap();

  let name_size = usize::from_be_bytes(name_size_buf);

  let mut name_buf = vec![0u8; name_size];
  reader.read_exact(&mut name_buf).unwrap();

  let name = String::from_utf8(name_buf);

  match name {
    Ok(name) => Some(name),
    Err(_) => None,
  }
}
