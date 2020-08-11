use std::io::{Read, ErrorKind};
use std::net::{TcpStream};

use crate::errors::Errors;

pub fn read_stream(client: &mut TcpStream, byte_amount: usize, chunk: usize) -> Result<Vec<u8>, Errors> {
  let mut data = Vec::with_capacity(byte_amount);
  let mut read_bytes = 0;

  while read_bytes < byte_amount {
    let mut buffer = Vec::with_capacity(chunk);
    let length = match client.read(&mut buffer) {
      Ok(l) => l,
      Err(e) => {
        println!("{}", e);
        return Err(Errors::ReadError("Failed to read bytes".to_string()));
      }
    };
    read_bytes += length;

    data.append(&mut buffer.to_vec());

    if length <= 0 {
      break;
    }
  }

  let slice = &data[0..byte_amount];

  return Ok(slice.to_vec());
}

/// Reads no more, but potentially less data than the byte_amount from the stream.
pub fn pluck_stream(client: &mut TcpStream, byte_amount: &u32) -> Result<Vec<u8>, Errors> {
  let mut data = Vec::with_capacity(*byte_amount as usize);

  match client.read(&mut data) {
    Ok(l) => l,
    Err(e) => {
      return match e.kind() {
        ErrorKind::Interrupted => Err(Errors::ReadRetryError),
        _ => Err(Errors::ReadError("Failed to pluck bytes".to_string()))
      };
    }
  };

  return Ok(data);
}