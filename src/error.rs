use std::error::Error;
use std::fmt;
use std::io;

/// Errors for the Arpabet crate.
#[derive(Debug)]
pub enum ArpabetError {
  /// The file or stream being read from was empty.
  EmptyFile,
  /// The file or stream contained invalid syntax.
  InvalidFormat {
    /// Line where the error occurred.
    line_number: usize,
    /// Text of the offending line.
    text: String,
  },
  /// An error during file IO.
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
