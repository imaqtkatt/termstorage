use termstorage_term::Term;

use std::io::{self, Read, Result, Write};

use byteorder::{NetworkEndian, ReadBytesExt, WriteBytesExt};

#[derive(Debug)]
pub enum Response {
  Ok(Term),
  Processed,
  NotFound,
  Unprocessed,
  ServerError,
}

const TAG_OK: u8 = 50;
const TAG_PROCESSED: u8 = 51;
const TAG_NOT_FOUND: u8 = 52;
const TAG_UNPROCESSED: u8 = 53;
const TAG_SERVER_ERROR: u8 = 54;

const USIZE_BYTES: usize = 8;

pub fn encode(resp: Response) -> Result<Vec<u8>> {
  let mut buf = Vec::new();

  match resp {
    Response::Ok(term) => {
      buf.write_u8(TAG_OK)?;
      let payload = termstorage_term::encode(&term)?;
      let payload_size = payload.len();

      buf.write_uint::<NetworkEndian>(payload_size as u64, USIZE_BYTES)?;
      buf.write(&payload)?;
    }
    Response::Processed => buf.write_u8(TAG_PROCESSED)?,
    Response::NotFound => buf.write_u8(TAG_NOT_FOUND)?,
    Response::Unprocessed => buf.write_u8(TAG_UNPROCESSED)?,
    Response::ServerError => buf.write_u8(TAG_SERVER_ERROR)?,
  };

  Ok(buf)
}

pub fn decode(reader: &mut impl Read) -> Result<Response> {
  let tag = reader.read_u8()?;

  match tag {
    TAG_OK => {
      let payload_size = reader.read_uint::<NetworkEndian>(USIZE_BYTES)?;

      let mut payload_buf = vec![0u8; payload_size as usize];
      reader.read_exact(&mut payload_buf)?;

      let term = termstorage_term::decode(&payload_buf)?;

      Ok(Response::Ok(term))
    }
    TAG_PROCESSED => Ok(Response::Processed),
    TAG_NOT_FOUND => Ok(Response::NotFound),
    TAG_UNPROCESSED => Ok(Response::Unprocessed),
    TAG_SERVER_ERROR => Ok(Response::ServerError),
    _ => {
      let err = io::Error::new(io::ErrorKind::InvalidData, "Invalid data");
      Err(err)
    }
  }
}
