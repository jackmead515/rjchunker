pub mod read;

use crate::pipe::Lease;

pub struct Headers {
  header_type: HeaderType,
  lease_id: Option<String>, 
  file_name: Option<String>,
  chunk_num: Option<u32>,
  chunk_length: Option<u32>,
  chunk_amount: Option<u32>,
  cancel: Option<bool>
}

pub enum HeaderType {
  LEASE,
  CHUNK,
  FINAL,
  CANCEL,
  ERROR,
}

pub fn get_header_type(headers: &Headers, lease: &Lease) -> HeaderType {
  if is_lease_type(headers) {
    return HeaderType::LEASE;
  }

  if is_chunk_type(headers) {
    return HeaderType::CHUNK;
  }

  if is_final_type(headers, lease) {
    return HeaderType::FINAL;
  }

  if is_cancel_type(headers) {
    return HeaderType::CANCEL;
  }

  return HeaderType::ERROR;
}

pub fn is_lease_type(headers: &Headers) -> bool {
  return headers.chunk_amount.is_some()
    && headers.chunk_length.is_some()
    && headers.chunk_num.is_some()
    && headers.file_name.is_some()
    && headers.cancel.is_none()
    && headers.lease_id.is_none();
}

pub fn is_chunk_type(headers: &Headers) -> bool {
  return headers.lease_id.is_some()
    && headers.chunk_num.is_some()
    && headers.file_name.is_none()
    && headers.cancel.is_none()
    && headers.chunk_amount.is_none()
    && headers.chunk_length.is_none()
}

pub fn is_final_type(headers: &Headers, lease: &Lease) -> bool {
  let last_chunk = match headers.chunk_num {
    Some(number) => lease.chunk_amount == number,
    None => false
  };

  return headers.lease_id.is_some()
    && last_chunk
    && headers.file_name.is_none()
    && headers.cancel.is_none()
    && headers.chunk_amount.is_none()
    && headers.chunk_length.is_none()
}

pub fn is_cancel_type(headers: &Headers) -> bool {
  return headers.cancel.is_some();
}




