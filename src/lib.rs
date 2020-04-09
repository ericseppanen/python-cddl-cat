use cpython::{
    py_exception, py_fn, py_module_initializer, PyResult, Python, PyObject
};

py_module_initializer!(cddlcat, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust.")?;
    m.add(
        py,
        "validate_cbor_bytes",
        py_fn!(
            py,
            validate_cbor_bytes(typename: &str, cddl: &str, cbor: &[u8])
        ),
    )?;
    m.add(
        py,
        "validate_json_str",
        py_fn!(
            py,
            validate_json_str(typename: &str, cddl: &str, json: &str)
        ),
    )?;
    // FIXME: ValidateError doesn't appear in the python module.  How do I add it?
    Ok(())
});

/// Validate a JSON string against a CDDL type specification.
fn validate_cbor_bytes(py: Python, typename: &str, cddl: &str, cbor: &[u8]) -> PyResult<PyObject> {
    cddl_cat::validate_cbor_bytes(typename, cddl, cbor).map_err(|e| {
        ValidateError::new(py, format!("{}", e))
    })?;
    Ok(py.None())
}

/// Validate a JSON string against a CDDL type specification.
fn validate_json_str(py: Python, typename: &str, cddl: &str, json: &str) -> PyResult<PyObject> {
    cddl_cat::validate_json_str(typename, cddl, json).map_err(|e| {
        ValidateError::new(py, format!("{}", e))
    })?;
    Ok(py.None())
}

py_exception!(cddlcat, ValidateError);
