// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
#![warn(clippy::all)]

/*!
Python interface to mwalib via pyo3.
 */
use crate::baseline::mwalibBaseline;
use crate::context::mwalibContext;
use crate::error::*;
use pyo3::prelude::*;

/// A Python class interfacing with the mwalib code written in Rust.
/* #[pyclass]
struct PymwalibBaseline {
    baseline: baseline_rust,
}

#[pymethods]
impl PymwalibBaseline {
    /// Create a new `baseline` object.
    #[new]
    fn new() -> PyResult<Self> {
        Ok(PymwalibBaseline {
            baseline: baseline_rust {
                antenna1_index: 0,
                antenna2_index: 0,
            },
        })
    }
}
 */
#[cfg_attr(feature = "python", pymodule)]
fn mwalib(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<mwalibContext>()?;
    m.add_class::<mwalibBaseline>()?;
    m.add("PymwalibError", py.get_type::<PymwalibError>())?;
    Ok(())
}
