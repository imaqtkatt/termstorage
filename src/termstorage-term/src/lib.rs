use std::io::{self, BufReader, BufWriter, Read, Result, Write};

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

const NUMBER_BYTES: usize = 8;
const USIZE_BYTES: usize = 8; // What if 32 bit?

/// Encodes the given term ordered by big endianness.
pub fn encode(term: &Term) -> Result<Vec<u8>> {
  let buf = Vec::new();

  let mut writer = BufWriter::new(buf);

  match term {
    Term::Bool(b) => {
      writer.write(&[TAG_BOOL])?;
      let byte = u8::from(*b);
      writer.write(&[byte])?;
    }
    Term::Number(n) => {
      writer.write(&[TAG_NUMBER])?;
      let bytes = n.to_be_bytes();
      writer.write(&bytes)?;
    }
    Term::String(s) => {
      writer.write(&[TAG_STRING])?;
      // How long strings can be?
      let str_len = s.len().to_be_bytes();
      writer.write(&str_len)?;
      let bytes = s.as_bytes();
      writer.write(bytes)?;
    }
    Term::Tuple(p0, p1) => {
      writer.write(&[TAG_TUPLE])?;
      let p0 = encode(p0)?;
      writer.write(&p0)?;
      let p1 = encode(p1)?;
      writer.write(&p1)?;
    }
  };

  let vec = writer.buffer().to_vec();
  Ok(vec)
}

/// Decodes the given slice.
pub fn decode(slice: &[u8]) -> Result<Term> {
  let mut reader = BufReader::new(slice);

  do_decode(&mut reader)
}

fn do_decode(reader: &mut BufReader<&[u8]>) -> Result<Term> {
  let mut buf = [0u8; 1];
  reader.read_exact(&mut buf).unwrap();

  match buf[0] {
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
  let mut buf = [0u8; 1];
  reader.read_exact(&mut buf)?;

  Ok(Term::Bool(buf[0] != 0))
}

fn decode_number(reader: &mut BufReader<&[u8]>) -> Result<Term> {
  let mut buf = [0u8; NUMBER_BYTES];
  reader.read_exact(&mut buf)?;
  let number = f64::from_be_bytes(buf);

  Ok(Term::Number(number))
}

fn decode_string(reader: &mut BufReader<&[u8]>) -> Result<Term> {
  let mut buf_usize = [0u8; USIZE_BYTES];
  reader.read_exact(&mut buf_usize)?;

  let str_size = usize::from_be_bytes(buf_usize);

  let mut buf = vec![0u8; str_size];
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
