use std::error::Error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum ArpabetError {
  EmptyFile,
  InvalidFormat {
    line_number: usize,
    text: String,
  },
  Io(io::Error),
}

impl fmt::Display for ArpabetError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      ArpabetError::EmptyFile => write!(f, "The file was empty."),
      ArpabetError::InvalidFormat { ref line_number, ref text } =>
          write!(f, "Invalid format on line {}: {}", line_number, text),
      ArpabetError::Io(ref err) => err.fmt(f),
    }
  }
}

impl Error for ArpabetError {
  fn description(&self) -> &str {
    match *self {
      ArpabetError::EmptyFile => "The file was empty.",
      ArpabetError::InvalidFormat { .. } => "Invalid format.",
      ArpabetError::Io(ref err) => err.description(),
    }
  }

  fn cause(&self) -> Option<&Error> {
    match *self {
      ArpabetError::EmptyFile => None,
      ArpabetError::InvalidFormat { .. } => None,
      ArpabetError::Io(ref err) => Some(err),
    }
  }
}

impl From<io::Error> for ArpabetError {
  fn from(err: io::Error) -> ArpabetError {
    ArpabetError::Io(err)
  }
}
