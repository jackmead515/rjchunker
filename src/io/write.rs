use std::io::Write;
use std::net::{TcpStream};

use crate::errors::Errors;

pub const OK_MESSAGE: [u8; 1] = [1 as u8];
pub const ERR_MESSAGE: [u8; 1] = [2 as u8];
pub const CONTINUE_MESSAGE: [u8; 1] = [3 as u8];
pub const RETRY_MESSAGE: [u8; 1] = [4 as u8];
pub const NO_LEASE_MESSAGE: [u8; 1] = [5 as u8];
pub const LEASE_IN_USE_MESSAGE: [u8; 1] = [6 as u8];

pub fn write_ok(client: &mut TcpStream) -> Result<(), Errors> {
  return match client.write_all(&OK_MESSAGE) {
    Ok(()) => Ok(()),
    Err(e) => Err(Errors::WriteError("Failed to write ok".to_string()))
  };
}

pub fn write_err(client: &mut TcpStream) -> Result<(), Errors> {
  return match client.write_all(&ERR_MESSAGE) {
    Ok(()) => Ok(()),
    Err(e) => Err(Errors::WriteError("Failed to write err".to_string()))
  };
}

pub fn write_continue(client: &mut TcpStream) -> Result<(), Errors> {
  return match client.write_all(&CONTINUE_MESSAGE) {
    Ok(()) => Ok(()),
    Err(e) => Err(Errors::WriteError("Failed to write continue".to_string()))
  };
}

pub fn write_retry(client: &mut TcpStream) -> Result<(), Errors> {
  return match client.write_all(&RETRY_MESSAGE) {
    Ok(()) => Ok(()),
    Err(e) => Err(Errors::WriteError("Failed to write retry".to_string()))
  };
}

pub fn write_no_lease(client: &mut TcpStream) -> Result<(), Errors> {
  return match client.write_all(&NO_LEASE_MESSAGE) {
    Ok(()) => Ok(()),
    Err(e) => Err(Errors::WriteError("Failed to write no lease".to_string()))
  };
}

pub fn write_lease_in_use(client: &mut TcpStream) -> Result<(), Errors> {
  return match client.write_all(&LEASE_IN_USE_MESSAGE) {
    Ok(()) => Ok(()),
    Err(e) => Err(Errors::WriteError("Failed to write lease in use".to_string()))
  };
}