use std::io::Write;
use std::sync::{Mutex, Arc};
use std::net::Shutdown;
use std::thread;
use std::fs::OpenOptions;
use num_cpus;
use crossbeam_channel::{unbounded, Sender, Receiver};

use crate::{Cache, Request};
use crate::headers::HeaderType;
use crate::errors::Errors;
use crate::io;

pub fn start(cache: Arc<Cache>, assembler_r: Receiver<Request>) {
  let worker_s = start_workers(cache);

  loop {
    if let Ok(request) = assembler_r.recv() {
      worker_s.send(request).expect("Unhandled sent to worker thread");
    }
  }
}

fn start_workers(cache: Arc<Cache>) -> Sender<Request> {
  let (worker_s, worker_r): (Sender<Request>, Receiver<Request>) = unbounded();
  let worker_r = Arc::new(Mutex::new(worker_r));
  let cores = num_cpus::get();

  for _ in 0..cores {
    let c = cache.clone();
    let r = worker_r.clone();
    thread::spawn(move || assembler(c, r));
  }

  return worker_s;
}

fn assembler(cache: Arc<Cache>, worker_r: Arc<Mutex<Receiver<Request>>>) {
  loop {
    let receiver = worker_r.lock().expect("Unhandled lock on worker receiver");
    let mut request = receiver.recv().expect("Unhandled worker receiver error");
    drop(receiver);

    let result = match request.headers.as_ref().unwrap().header_type {
      HeaderType::LEASE => handle_lease_request(&cache, &mut request),
      HeaderType::CHUNK => handle_chunk_request(&cache, &mut request),
      HeaderType::CANCEL => handle_cancel_request(&cache, &mut request),
      HeaderType::FINAL => handle_final_request(&cache, &mut request),
      HeaderType::ERROR => {
        // dunno what you are...
        // shouldn't happen. But kill that client!
        Err(Errors::InvalidRequest("Header Type is Error".to_string()))
      },
    };

    match result {
      Ok(_) => {
        io::write::write_ok(&mut request.client).ok();
        request.client.shutdown(Shutdown::Both).ok();
      },
      Err(e) => {
        println!("{:?}", e);
        io::write::write_err(&mut request.client).ok();
        request.client.shutdown(Shutdown::Both).ok();
      }
    };
  }
}

fn handle_lease_request(cache: &Arc<Cache>, request: &mut Request) -> Result<(), Errors> {
  // TODO
  // create a new file. Check if exists? Shouldnt exist because it's a new uuid
  // write chunk to file.
  // edit and save the lease in cache
  // respond with lease id to client

  let lease = request.lease.as_mut().unwrap();
  let headers = request.headers.as_ref().unwrap();
  let file_name = headers.file_name.as_ref().unwrap();
  let lease_id = headers.lease_id.as_ref().unwrap();
  let chunk_length = headers.chunk_length.as_ref().unwrap();
  let chunk_num = headers.chunk_num.as_ref().unwrap();
  let chunk = request.chunk.as_ref().unwrap();

  let file_location = format!("{}_{}", lease_id, file_name);

  let chunk_num_bytes: [u8; 4] = chunk_num.to_le_bytes();
  let chunk_length_bytes: [u8; 4] = chunk_length.to_le_bytes();
  let mut package: Vec<u8> = Vec::with_capacity(chunk.len() + 4 + 4);
  package.extend_from_slice(&chunk_num_bytes);
  package.extend_from_slice(&chunk_length_bytes);
  package.extend_from_slice(&chunk);

  let result = OpenOptions::new()
    .append(true)
    .create(true)
    .open(file_location)
    .and_then(|mut file| {
      file.write_all(&package)?;
      file.flush()?;
      return Ok(());
    });
  
  if result.is_err() {
    return Err(Errors::FileIOError("Failed to write chunk to file".to_string()));
  }

  lease.bytes_left = lease.file_length - chunk_length;
  lease.chunks_sent += 1;
  lease.in_use = false;

  if let Ok(mut leases) = cache.leases.lock() {
    leases.insert(lease_id.to_string(), lease.clone());
  } else {
    return Err(Errors::UnexpectedError("Failed to get lock on cached leases".to_string()));
  }

  io::write::write_string(&mut request.client, lease_id)?;

  return Ok(());
}

fn handle_chunk_request(cache: &Arc<Cache>, request: &mut Request) -> Result<(), Errors> {
  // TODO
  // write chunk to file
  // edit and save the lease in cache
  // respond success to client
  // close connection
  return Ok(());
}

fn handle_cancel_request(cache: &Arc<Cache>, request: &mut Request) -> Result<(), Errors> {
  // TODO
  // remove file from database
  // delete lease in cache
  // respond success to client
  // close connection
  return Ok(());
}

fn handle_final_request(cache: &Arc<Cache>, request: &mut Request) -> Result<(), Errors> {
  // TODO
  // write chunk to file
  // rearrange file chunks. Maybe a background process? Might take some time
  // delete lease in cache
  // respond success to client
  // close connection
  return Ok(());
}