use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
/// Validate a JSON string against a CDDL type specification.
fn validate_cbor_bytes(typename: &str, cddl: &str, cbor: &[u8]) -> bool {
    let res = cddl_cat::validate_cbor_bytes(cddl, typename, cbor);
    match res {
        Ok(()) => true,
        Err(_e) => false,
    }
}


#[pyfunction]
/// Validate a JSON string against a CDDL type specification.
fn validate_json_str(typename: &str, cddl: &str, json: &str) -> bool {
    let res = cddl_cat::validate_json_str(cddl, typename, json);
    match res {
        Ok(()) => true,
        Err(_e) => false,
    }
}

/// Produce a python module with the chosen functions exposed.
#[pymodule]
fn cddlcat(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(validate_cbor_bytes))?;
    m.add_wrapped(wrap_pyfunction!(validate_json_str))?;

    Ok(())
}
