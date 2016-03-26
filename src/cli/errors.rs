use std::io;
use std::error;
use std::fmt;
use std::convert::From;
use std::error::Error;

#[derive(Debug)]
pub enum CliError {
    Io(io::Error),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CliError::Io(ref err) => write!(f, "IO error: {}", err),
        }
    }
}

impl error::Error for CliError {
    fn description(&self) -> &str {
        match *self {
            CliError::Io(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
         match *self {
             CliError::Io(ref err) => Some(err),
         }
    }
}

impl From<io::Error> for CliError {
    fn from(error: io::Error) -> CliError {
       CliError::Io(error)
    }
}
