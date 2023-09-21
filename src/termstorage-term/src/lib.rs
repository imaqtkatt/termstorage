use std::io::{self, BufReader, Read, Result, Write};

use byteorder::{NetworkEndian, ReadBytesExt, WriteBytesExt};

#[derive(Debug, Clone)]
pub enum Term {
  Bool(bool),
  Number(f64),
  String(String),
  Tuple(Box<Term>, Box<Term>),
}

const TAG_BOOL: u8 = 20;
const TAG_NUMBER: u8 = 21;
const TAG_STRING: u8 = 22;
const TAG_TUPLE: u8 = 23;

const USIZE_BYTES: usize = 8; // What if 32 bit?

/// Encodes the given term ordered by big endianness.
pub fn encode(term: &Term) -> Result<Vec<u8>> {
  let mut buf = Vec::new();

  match term {
    Term::Bool(b) => {
      buf.write_u8(TAG_BOOL)?;
      let byte = u8::from(*b);
      buf.write_u8(byte)?;
    }
    &Term::Number(n) => {
      buf.write_u8(TAG_NUMBER)?;
      buf.write_f64::<NetworkEndian>(n)?;
    }
    Term::String(s) => {
      buf.write_u8(TAG_STRING)?;
      // How long strings can be?
      let str_len = s.len();
      buf.write_uint::<NetworkEndian>(str_len as u64, USIZE_BYTES)?;
      let bytes = s.as_bytes();
      buf.write(bytes)?;
    }
    Term::Tuple(p0, p1) => {
      buf.write_u8(TAG_TUPLE)?;
      let p0 = encode(p0)?;
      buf.write(&p0)?;
      let p1 = encode(p1)?;
      buf.write(&p1)?;
    }
  };

  Ok(buf)
}

/// Decodes the given slice to a term.
pub fn decode(slice: &[u8]) -> Result<Term> {
  let mut reader = BufReader::new(slice);

  do_decode(&mut reader)
}

fn do_decode(reader: &mut BufReader<&[u8]>) -> Result<Term> {
  let tag = reader.read_u8()?;

  match tag {
    TAG_BOOL => decode_bool(reader),
    TAG_NUMBER => decode_number(reader),
    TAG_STRING => decode_string(reader),
    TAG_TUPLE => decode_tuple(reader),
    _ => {
      let err = io::Error::new(io::ErrorKind::InvalidData, "Invalid tag");
      Err(err)
    }
  }
}

fn decode_bool(reader: &mut BufReader<&[u8]>) -> Result<Term> {
  let bool = reader.read_u8()?;

  Ok(Term::Bool(bool != 0))
}

fn decode_number(reader: &mut BufReader<&[u8]>) -> Result<Term> {
  let number = reader.read_f64::<NetworkEndian>()?;

  Ok(Term::Number(number))
}

fn decode_string(reader: &mut BufReader<&[u8]>) -> Result<Term> {
  let str_size = reader.read_uint::<NetworkEndian>(USIZE_BYTES)?;

  let mut buf = vec![0u8; str_size as usize];
  reader.read_exact(&mut buf)?;

  let string = String::from_utf8(buf);

  match string {
    Ok(s) => Ok(Term::String(s)),
    Err(e) => {
      let err = io::Error::new(io::ErrorKind::InvalidInput, e);
      Err(err)
    }
  }
}

fn decode_tuple(reader: &mut BufReader<&[u8]>) -> Result<Term> {
  let p0 = do_decode(reader)?;
  let p1 = do_decode(reader)?;

  Ok(Term::Tuple(Box::new(p0), Box::new(p1)))
}
