use std::net::TcpStream;

use crate::errors::Errors;
use crate::io::{read, util};
use crate::headers::{Headers, HeaderType, MIN_CHUNK_BYTES, MAX_CHUNK_BYTES};

pub const UUID_BYTES: u32 = 16;
pub const UUID_POS: u8 = 0;

pub const CHECKSUM_BYTES: u32 = 128;
pub const CHECKSUM_POS: u8 = 1;

pub const FILE_NAME_BYTES: u32 = 128;
pub const FILE_NAME_POS: u8 = 2;

pub const FILE_LENGTH_BYTES: u32 = 4;
pub const FILE_LENGTH_POS: u8 = 3;

pub const CHUNK_LENGTH_BYTES: u32 = 4;
pub const CHUNK_LENGTH_POS: u8 = 4;

pub const CANCEL_POS: u8 = 5;

pub fn read_headers(client: &mut TcpStream) -> Result<Headers, Errors> {
  let params = read::pluck_stream(client, &1)?[0];

  let mut headers = Headers {
    header_type: HeaderType::ERROR,
    lease_id: None,
    checksum: None,
    file_name: None,
    file_length: None,
    chunk_length: None,
    cancel: None
  };

  if util::bit_at(params, UUID_POS) {
    let data = read::pluck_stream(client, &UUID_BYTES)?;
    if data.len() != UUID_BYTES as usize {
      return Err(Errors::ReadError("invalid uuid length from headers".to_string()));
    }
    headers.lease_id = Some(String::from_utf8_lossy(&data).to_string());
  }

  if util::bit_at(params, CHECKSUM_POS) {
    let data = read::pluck_stream(client, &CHECKSUM_BYTES)?;
    if data.len() != CHECKSUM_BYTES as usize {
      return Err(Errors::ReadError("invalid checksum length from headers".to_string()));
    }
    headers.checksum = Some(String::from_utf8_lossy(&data).to_string());
  }

  if util::bit_at(params, FILE_NAME_POS) {
    let data = read::pluck_stream(client, &FILE_NAME_BYTES)?;
    if data.len() != FILE_NAME_BYTES as usize {
      return Err(Errors::ReadError("invalid file_name length from headers".to_string()));
    }
    headers.file_name = Some(String::from_utf8_lossy(&data).to_string());
  }

  if util::bit_at(params, FILE_LENGTH_POS) {
    let data = read::pluck_stream(client, &FILE_LENGTH_BYTES)?;
    if data.len() != FILE_LENGTH_BYTES as usize {
      return Err(Errors::ReadError("invalid file_length length from headers".to_string()));
    }
    headers.file_length = Some(util::read_u32(data)?);
  }

  if util::bit_at(params, CHUNK_LENGTH_POS) {
    let data = read::pluck_stream(client, &CHUNK_LENGTH_BYTES)?;
    if data.len() != CHUNK_LENGTH_BYTES as usize {
      return Err(Errors::ReadError("invalid chunk_length length from headers".to_string()));
    }
    let chunk_length = util::read_u32(data)?;

    if chunk_length > MAX_CHUNK_BYTES {
      return Err(Errors::InvalidRequest("requests chunk length to large".to_string()));
    }

    if chunk_length < MIN_CHUNK_BYTES {
      return Err(Errors::InvalidRequest("requests chunk length to small".to_string()));
    }

    headers.chunk_length = Some(chunk_length);
  }

  if util::bit_at(params, CANCEL_POS) {
    headers.cancel = Some(true);
  }

  return Ok(headers);
}

