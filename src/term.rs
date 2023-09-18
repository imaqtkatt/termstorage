use std::io::{BufReader, Read};

#[derive(Debug)]
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

const NUMBER_LENGTH: usize = 8;
const USIZE_LENGTH: usize = 8; // What if 32 bit?

/// Encodes the given term ordered by big endianness.
pub fn encode(term: &Term) -> Vec<u8> {
  let mut encd = Vec::new();

  match term {
    Term::Bool(b) => {
      encd.push(TAG_BOOL);
      let byte = u8::from(*b);
      encd.push(byte);
    }
    Term::Number(n) => {
      encd.push(TAG_NUMBER);
      let bytes = n.to_be_bytes();
      encd.extend_from_slice(&bytes);
    }
    Term::String(s) => {
      encd.push(TAG_STRING);
      // How long strings can be?
      let str_len = s.len().to_be_bytes();
      encd.extend_from_slice(&str_len);
      let bytes = s.as_bytes();
      encd.extend_from_slice(bytes);
    }
    Term::Tuple(p0, p1) => {
      encd.push(TAG_TUPLE);
      let p0 = encode(p0);
      encd.extend(p0);
      let p1 = encode(p1);
      encd.extend(p1);
    }
  };

  encd
}

/// Decodes the given slice.
pub fn decode(slice: &[u8]) -> Option<Term> {
  let mut reader = BufReader::new(slice);

  do_decode(&mut reader)
}

fn do_decode(reader: &mut BufReader<&[u8]>) -> Option<Term> {
  let mut buf = [0u8; 1];
  reader.read_exact(&mut buf).unwrap();

  match buf[0] {
    TAG_BOOL => decode_bool(reader),
    TAG_NUMBER => decode_number(reader),
    TAG_STRING => decode_string(reader),
    TAG_TUPLE => decode_tuple(reader),
    _ => None,
  }
}

fn decode_bool(reader: &mut BufReader<&[u8]>) -> Option<Term> {
  let mut buf = [0u8; 1];
  reader.read_exact(&mut buf).unwrap();

  Some(Term::Bool(buf[0] != 0))
}

fn decode_number(reader: &mut BufReader<&[u8]>) -> Option<Term> {
  let mut buf = [0u8; NUMBER_LENGTH];
  reader.read_exact(&mut buf).unwrap();
  let number = f64::from_be_bytes(buf);

  Some(Term::Number(number))
}

fn decode_string(reader: &mut BufReader<&[u8]>) -> Option<Term> {
  let mut buf_usize = [0u8; USIZE_LENGTH];
  reader.read_exact(&mut buf_usize).unwrap();

  let str_size = usize::from_be_bytes(buf_usize);

  let mut buf = vec![0u8; str_size];
  reader.read_exact(&mut buf).unwrap();

  let s = String::from_utf8(buf);

  match s {
    Ok(s) => Some(Term::String(s)),
    Err(_) => None,
  }
}

fn decode_tuple(reader: &mut BufReader<&[u8]>) -> Option<Term> {
  let p0 = do_decode(reader)?;
  let p1 = do_decode(reader)?;

  Some(Term::Tuple(Box::new(p0), Box::new(p1)))
}
