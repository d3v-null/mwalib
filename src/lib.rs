// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[macro_use]
extern crate lazy_static;

pub mod context;
pub mod error;
pub mod ffi;
pub mod fits_read;
pub mod gpubox;
pub mod metadata;
pub mod misc;

// Re-exports.
use anyhow::Context;
pub use context::mwalibContext;
pub use error::ErrorKind;
pub use metadata::CorrelatorVersion;
