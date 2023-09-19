use termstorage_term::Term;

use std::io::{self, BufWriter, Read, Result, Write};

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

pub fn encode(resp: Response) -> Result<Vec<u8>> {
  let buf = Vec::new();

  let mut writer = BufWriter::new(buf);

  match resp {
    Response::Ok(term) => {
      writer.write(&[TAG_OK])?;
      let payload = termstorage_term::encode(&term)?;
      let payload_size = payload.len().to_be_bytes();

      writer.write(&payload_size)?;
      writer.write(&payload)?
    }
    Response::Processed => writer.write(&[TAG_PROCESSED])?,
    Response::NotFound => writer.write(&[TAG_NOT_FOUND])?,
    Response::Unprocessed => writer.write(&[TAG_UNPROCESSED])?,
    Response::ServerError => writer.write(&[TAG_SERVER_ERROR])?,
  };

  let vec = writer.buffer().to_vec();
  Ok(vec)
}

pub fn decode(reader: &mut impl Read) -> Result<Response> {
  // let mut reader = BufReader::new(slice);

  let mut buf = [0u8; 1];
  reader.read_exact(&mut buf)?;

  match buf[0] {
    TAG_OK => {
      let mut payload_size_buf = [0u8; 8];
      reader.read_exact(&mut payload_size_buf)?;

      let payload_size = usize::from_be_bytes(payload_size_buf);

      let mut payload_buf = vec![0u8; payload_size];
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
