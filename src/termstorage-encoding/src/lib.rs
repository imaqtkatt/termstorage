use std::io::Read;

/// Provides a method to encode a structure to a sequence of bytes.
pub trait Encode {
  fn encode(&self) -> std::io::Result<Vec<u8>>;
}

/// Provides a function to decode a sequence of bytes to a structure.
pub trait Decode: Sized {
  fn decode(rd: &mut impl Read) -> std::io::Result<Self>;
}
