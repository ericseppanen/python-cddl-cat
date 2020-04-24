use pyo3::prelude::*;
use pyo3::{create_exception, wrap_pyfunction};

#[pymodule]
fn cddlcat(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(validate_cbor_bytes))
        .unwrap();
    m.add_wrapped(wrap_pyfunction!(validate_json_str)).unwrap();
    Ok(())
}

create_exception!(cddlcat, ValidateError, pyo3::exceptions::Exception);

fn err_adapter(err: cddl_cat::util::ValidateError) -> PyErr {
    ValidateError::py_err(format!("{}", err))
}

/// Validate a JSON string against a CDDL type specification.
#[pyfunction]
fn validate_cbor_bytes(typename: &str, cddl: &str, cbor: &[u8]) -> PyResult<()> {
    cddl_cat::validate_cbor_bytes(typename, cddl, cbor).map_err(err_adapter)?;
    Ok(())
}

/// Validate a JSON string against a CDDL type specification.
#[pyfunction]
fn validate_json_str(typename: &str, cddl: &str, json: &str) -> PyResult<()> {
    cddl_cat::validate_json_str(typename, cddl, json).map_err(err_adapter)?;
    Ok(())
}
