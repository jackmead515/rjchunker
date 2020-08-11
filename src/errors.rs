#[derive(Debug)]
pub enum Errors {
  ReadError(String),
  ReadLengthError(String),
  ReadRetryError,
  WriteError(String),
  ParseError(String),
  InvalidRequest(String),
  FileIOError(String),
  UnexpectedError(String),
}