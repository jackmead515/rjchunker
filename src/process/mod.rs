use std::sync::Arc;
use std::net::{TcpStream, Shutdown};
use crossbeam_channel::{Sender, Receiver};
use uuid::Uuid;

use crate::{Request, Cache};
use crate::headers::read::read_headers;
use crate::headers::HeaderType;
use crate::errors::Errors;
use crate::io;

#[derive(Debug, Clone)]
pub struct Lease {
  pub id: String,
  pub file_name: String,
  pub hash: String,
  pub file_length: u32,
  pub bytes_left: u32,
  pub chunks_sent: u32,
  pub ns_last_sent: u128,
  pub in_use: bool
}

pub fn start(cache: Arc<Cache>, process_r: Receiver<Request>, assembler_s: Sender<Request>) {
  loop {
    if let Ok(mut request) = process_r.recv() {

      match get_request_headers(&mut request) {
        Ok(()) => (),
        Err(e) => {
          println!("{:?}", e);
          io::write::write_err(&mut request.client).ok();
          request.client.shutdown(Shutdown::Both).ok();
          continue;
        }
      }

      let result = match request.headers.as_ref().unwrap().header_type {
        HeaderType::LEASE => handle_lease_request(&mut request, &cache),
        HeaderType::CHUNK => handle_chunk_request(&mut request, &cache),
        HeaderType::CANCEL => handle_cancel_request(&mut request, &cache),
        HeaderType::FINAL => {
          // not possible...
          Err(Errors::UnexpectedError("this is impossible...".to_string()))
        },
        HeaderType::ERROR => {
          // dunno what you are...
          // shouldn't happen. But kill that client!
          Err(Errors::InvalidRequest("Header Type is Error".to_string()))
        },
      };

      match result {
        Ok(send_to_assembler) => {
          if send_to_assembler {
            assembler_s.send(request).expect("Unhandled assembler channel error");
          }
        },
        Err(e) => {
          println!("{:?}", e);
          io::write::write_err(&mut request.client).ok();
          request.client.shutdown(Shutdown::Both).ok();
        }
      };
    }
  }
}

fn get_request_headers(request: &mut Request) -> Result<(), Errors> {
  let mut headers = read_headers(&mut request.client)?;

  if headers.is_cancel_type() {
    headers.set_header_type(HeaderType::CANCEL);
  } else if headers.is_lease_type() {
    headers.set_header_type(HeaderType::LEASE);
  } else if headers.is_chunk_type() {
    headers.set_header_type(HeaderType::CHUNK);
  }

  request.headers = Some(headers);

  return Ok(());
}

fn handle_lease_request(request: &mut Request, cache: &Arc<Cache>) -> Result<bool, Errors> {
  let lease_id = Uuid::new_v4().to_string();
  let headers = request.headers.as_ref().unwrap();
  let checksum = headers.checksum.as_ref().unwrap();
  let file_name = headers.file_name.as_ref().unwrap();
  let file_length = headers.file_length.as_ref().unwrap();
  let chunk_length = headers.chunk_length.as_ref().unwrap();

  let lease = Lease {
    id: lease_id.to_string(),
    hash: checksum.to_string(),
    file_name: file_name.to_string(),
    file_length: *file_length,
    bytes_left: 0,
    chunks_sent: 0,
    ns_last_sent: 0,
    in_use: true
  };

  if let Ok(mut leases) = cache.leases.lock() {
    leases.insert(lease_id, lease.clone());
  } else {
    return Err(Errors::UnexpectedError("Failed to get lock on cached leases".to_string()));
  }

  request.lease = Some(lease);
  request.chunk = Some(read_retry_chunk(&mut request.client, chunk_length)?);
  request.set_last_chunk_time();

  return Ok(true);
}

fn handle_chunk_request(request: &mut Request, cache: &Arc<Cache>) -> Result<bool, Errors> {
  let headers = request.headers.as_ref().unwrap();
  let lease_id = headers.lease_id.as_ref().unwrap();
  let chunk_length = headers.chunk_length.as_ref().unwrap();

  if let Ok(leases) = cache.leases.lock() {
    request.lease = match leases.get(lease_id) {
      Some(l) => {
        if l.in_use {
          io::write::write_lease_in_use(&mut request.client)?;
          request.client.shutdown(Shutdown::Both).ok();
          return Ok(false);
        }

        Some(l.clone())
      },
      None => None
    }
  }

  if request.lease.is_none() {
    // KILL THAT CLIENT BOOOOYYY!
    // Or lease could have expired.
    io::write::write_no_lease(&mut request.client)?;
    request.client.shutdown(Shutdown::Both).ok();
    return Ok(false);
  }

  request.chunk = Some(read_retry_chunk(&mut request.client, chunk_length)?);
  if headers.is_final_type(&request.lease.as_ref().unwrap().bytes_left) {
    request.set_header_type(HeaderType::FINAL);
  }
  request.set_last_chunk_time();

  return Ok(true);
}

fn handle_cancel_request(request: &mut Request, cache: &Arc<Cache>) -> Result<bool, Errors> {
  let lease_id = request.headers.as_ref().unwrap().lease_id.as_ref().unwrap();

  let mut leases = cache.leases.lock().expect("Unhandled cache lease lock");
  request.lease = match leases.get(lease_id) {
    Some(lease) => {
      if lease.in_use {
        io::write::write_lease_in_use(&mut request.client)?;
        request.client.shutdown(Shutdown::Both).ok();
        return Ok(false);
      }
      
      leases.remove(lease_id)
    },
    None => None
  };

  if request.lease.is_none() {
    // bad client... or something
    // lease is already removed. 
    // Don't worry bout it. Or worry bout it?
    io::write::write_ok(&mut request.client)?;
    request.client.shutdown(Shutdown::Both).ok();
    return Ok(false);
  }

  return Ok(true);
}

fn read_retry_chunk(client: &mut TcpStream, chunk_length: &u32) -> Result<Vec<u8>, Errors> {
  let mut chunk: Option<Vec<u8>> = None;
  let mut retries = 0;

  while retries < 3 {
    match io::read::pluck_stream(client, chunk_length) {
      Ok(c) => {
        if c.len() != *chunk_length as usize {
          return Err(Errors::ReadLengthError("Failed to read entire chunk length".to_string()));
        }

        chunk = Some(c);
        break;
      },
      Err(e) => {
        match e {
          Errors::ReadRetryError => {
            retries += 1;
          },
          _ => break
        }
      }
    };
  }

  if chunk.is_some() {
    return Ok(chunk.unwrap());
  }

  return Err(Errors::ReadError("Failed to read in chunk".to_string()));
}