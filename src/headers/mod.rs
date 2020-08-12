pub mod read;

pub const MIN_CHUNK_BYTES: u32 = 1000; // 1 KB
pub const MAX_CHUNK_BYTES: u32 = 1000000; // 1 MB

#[derive(Debug)]
pub struct Headers {
  pub header_type: HeaderType,
  pub lease_id: Option<String>,
  pub checksum: Option<String>,
  pub file_name: Option<String>,
  pub file_length: Option<u32>,
  pub chunk_length: Option<u32>,
  pub chunk_num: Option<u32>,
  pub cancel: Option<bool>
}

impl Headers {
  pub fn set_header_type(&mut self, header_type: HeaderType) {
    self.header_type = header_type;
  }
}

#[derive(Debug, Clone, Copy)]
pub enum HeaderType {
  LEASE,
  CHUNK,
  FINAL,
  CANCEL,
  ERROR,
}

impl Headers {
  /// true if file_length, chunk_length, file_name, 
  /// chunk_num, and checksum is specified
  pub fn is_lease_type(&self) -> bool {
    return self.file_length.is_some()
      && self.chunk_length.is_some()
      && self.chunk_num.is_some()
      && self.file_name.is_some()
      && self.checksum.is_some()
      && self.cancel.is_none()
      && self.lease_id.is_none();
  }

  /// true if lease_id and chunk_length are specified
  pub fn is_chunk_type(&self) -> bool {
    return self.lease_id.is_some()
      && self.chunk_length.is_some()
      && self.chunk_num.is_some()
      && self.file_name.is_none()
      && self.cancel.is_none()
      && self.file_length.is_none()
  }

  /// true if the chunk_length equals the amount of bytes
  /// left in the lease. Keep in might a final type is also
  /// a chunk type.
  pub fn is_final_type(&self, bytes_left: &u32) -> bool {
    if let Some(chunk_length) = self.chunk_length {
      return bytes_left == &chunk_length;
    };

    return false;
  }

  /// true if cancel is specified
  pub fn is_cancel_type(&self) -> bool {
    return self.cancel.is_some()
      && self.lease_id.is_some()
  }
}



