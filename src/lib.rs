//! fileshiftlib - SFTP client powered by Rust and PyO3
//!
//! A high-performance SFTP client library for Python, implemented in Rust using PyO3.

mod models;

#[cfg(feature = "python")]
mod python_bindings;

pub use models::{SftpClient, SftpConfiguration, SftpError};

#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
#[pymodule]
fn fileshiftlib(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    m.add_class::<python_bindings::PySftpClient>()?;
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    let doc = "SFTP client powered by Rust and PyO3.";
    m.add("__doc__", doc)?;

    Ok(())
}
