// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
#![warn(clippy::all)]

/*!
Structs and helper methods for Error handling
*/
use thiserror::Error;

#[cfg(feature = "python")]
use pyo3::create_exception;
#[cfg(feature = "python")]
use pyo3::prelude::*;

#[derive(Error, Debug)]
pub enum MwalibError {
    /// An error derived from `FitsError`.
    #[error("{0}")]
    Fits(#[from] crate::fits_read::error::FitsError),

    /// An error derived from `RfinputError`.
    #[error("{0}")]
    Rfinput(#[from] crate::rfinput::error::RfinputError),

    /// An error derived from `GpuboxError`.
    #[error("{0}")]
    Gpubox(#[from] crate::gpubox::error::GpuboxError),

    /// An error associated with parsing a string into another type.
    #[error("{source_file}:{source_line}\nCouldn't parse {key} in {fits_filename} HDU {hdu_num}")]
    Parse {
        key: String,
        fits_filename: String,
        hdu_num: usize,
        source_file: String,
        source_line: u32,
    },
}

// Add a python exception for MwalibError.
#[cfg(feature = "python")]
create_exception!(mwalib, PymwalibError, pyo3::exceptions::PyException);

#[cfg(feature = "python")]
impl std::convert::From<MwalibError> for PyErr {
    fn from(err: MwalibError) -> PyErr {
        PymwalibError::new_err(err.to_string())
    }
}
