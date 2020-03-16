// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
/*!
Structs and helper methods for Error handling
*/
use std::error::Error;
use std::fmt;
use std::io;
use std::num;

#[derive(Debug)]
pub enum ErrorKind {
    Custom(String),
    ParseInt(num::ParseIntError),
    ParseFloat(num::ParseFloatError),
    IO(io::Error),
    Anyhow(anyhow::Error),
    Fitsio(fitsio::errors::Error),
}

impl From<num::ParseIntError> for ErrorKind {
    fn from(err: num::ParseIntError) -> ErrorKind {
        ErrorKind::ParseInt(err)
    }
}

impl From<num::ParseFloatError> for ErrorKind {
    fn from(err: num::ParseFloatError) -> ErrorKind {
        ErrorKind::ParseFloat(err)
    }
}

impl From<io::Error> for ErrorKind {
    fn from(err: io::Error) -> ErrorKind {
        ErrorKind::IO(err)
    }
}

impl From<anyhow::Error> for ErrorKind {
    fn from(err: anyhow::Error) -> ErrorKind {
        ErrorKind::Anyhow(err)
    }
}

impl From<fitsio::errors::Error> for ErrorKind {
    fn from(err: fitsio::errors::Error) -> ErrorKind {
        ErrorKind::Fitsio(err)
    }
}

impl Error for ErrorKind {}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ErrorKind::Custom(ref err) => err.fmt(f),
            ErrorKind::ParseInt(ref err) => err.fmt(f),
            ErrorKind::ParseFloat(ref err) => err.fmt(f),
            ErrorKind::IO(ref err) => err.fmt(f),
            ErrorKind::Anyhow(ref err) => err.fmt(f),
            ErrorKind::Fitsio(ref err) => err.fmt(f),
        }
    }
}
