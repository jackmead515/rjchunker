use std::net::TcpStream;

use crate::errors::Errors;
use crate::io::{read, util};
use crate::headers::{Headers, HeaderType};

pub const UUID_BYTES: usize = 16;
pub const UUID_POS: u8 = 0;

pub const CHUNK_BYTES: usize = 4;
pub const CHUNK_POS: u8 = 1;

pub const CHUNK_LENGTH_BYTES: usize = 4;
pub const CHUNK_LENGTH_POS: u8 = 2;

pub const CHUNK_AMOUNT_BYTES: usize = 4;
pub const CHUNK_AMOUNT_POS: u8 = 3;

pub const FILE_NAME_BYTES: usize = 128;
pub const FILE_NAME_POS: u8 = 4;

pub const CANCEL_BYTES: usize = 4;
pub const CANCEL_POS: u8 = 5;

pub fn read_headers(client: &mut TcpStream) -> Result<Headers, Errors> {
  let params = read::pluck_stream(client, 1)?[0];

  let mut headers = Headers {
    header_type: HeaderType::ERROR,
    lease_id: None, 
    file_name: None,
    chunk_num: None,
    chunk_length: None,
    chunk_amount: None,
    cancel: None
  };

  if util::bit_at(params, UUID_POS) {
    let data = read::pluck_stream(client, UUID_BYTES)?;
    headers.lease_id = Some(String::from_utf8_lossy(&data).to_string());
  }

  if util::bit_at(params, CHUNK_POS) {
    let data = read::pluck_stream(client, CHUNK_BYTES)?;
    headers.chunk_num = Some(util::read_u32(data)?);
  }

  if util::bit_at(params, CHUNK_LENGTH_POS) {
    let data = read::pluck_stream(client, CHUNK_LENGTH_BYTES)?;
    headers.chunk_length = Some(util::read_u32(data)?);
  }

  if util::bit_at(params, CHUNK_AMOUNT_POS) {
    let data = read::pluck_stream(client, CHUNK_AMOUNT_BYTES)?;
    headers.chunk_amount = Some(util::read_u32(data)?);
  }

  if util::bit_at(params, FILE_NAME_POS) {
    let data = read::pluck_stream(client, FILE_NAME_BYTES)?;
    headers.file_name = Some(String::from_utf8_lossy(&data).to_string());
  }

  if util::bit_at(params, CANCEL_POS) {
    headers.cancel = Some(true);
  }

  return Ok(headers);
}

