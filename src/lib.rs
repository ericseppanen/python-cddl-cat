use cddl_cat::ivt::Node;
use pyo3::exceptions::{AttributeError, IndexError};
use pyo3::prelude::*;
use pyo3::{
    create_exception, wrap_pyfunction, PyMappingProtocol, PyObjectProtocol, PySequenceProtocol,
};
use std::collections::BTreeMap;
use std::convert::TryInto;

// Register all the things that should be visible from python.
#[pymodule]
fn cddlcat(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(validate_cbor_bytes))
        .unwrap();
    m.add_wrapped(wrap_pyfunction!(validate_json_str)).unwrap();
    m.add_wrapped(wrap_pyfunction!(flatten_from_str)).unwrap();
    m.add_class::<IVTNode>()?;
    Ok(())
}

// Create a python-visible exception type.
create_exception!(cddlcat, ValidateError, pyo3::exceptions::Exception);

// Unfortunately, trait rules prevent us from implementing From<ValidateError> for PyErr,
// so we will finesse the errors by hand for now.
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

// A Python wrapper around the IVT Node.
#[pyclass]
struct IVTNode {
    node: Node,
}

impl From<Node> for IVTNode {
    fn from(node: Node) -> IVTNode {
        IVTNode { node }
    }
}

// Convert a Node::Literal into a Python object of the expected type.
fn literal_value(lit: &cddl_cat::ivt::Literal) -> PyResult<PyObject> {
    use cddl_cat::ivt::Literal::*;

    let gil = Python::acquire_gil();
    let py = gil.python();

    match lit {
        Bool(b) => Ok(b.to_object(py)),
        Int(i) => Ok(i.to_object(py)),
        Float(f) => Ok(f.to_object(py)),
        Text(s) => Ok(s.to_object(py)),
        Bytes(b) => Ok(b.to_object(py)),
    }
}

#[pymethods]
impl IVTNode {
    // Return a constant string as a way of identifying which variant.
    fn kind(&self) -> PyResult<String> {
        use Node::*;
        match self.node {
            Literal(_) => Ok("Literal".into()),
            PreludeType(_) => Ok("PreludeType".into()),
            Rule(_) => Ok("Rule".into()),
            Choice(_) => Ok("Choice".into()),
            Map(_) => Ok("Map".into()),
            Array(_) => Ok("Array".into()),
            Group(_) => Ok("Group".into()),
            KeyValue(_) => Ok("KeyValue".into()),
            Occur(_) => Ok("Occur".into()),
            Unwrap(_) => Ok("Unwrap".into()),
            Range(_) => Ok("Range".into()),
        }
    }

    // For variants that have a single value, return it.
    fn value(&self) -> PyResult<PyObject> {
        use Node::*;

        let gil = Python::acquire_gil();
        let py = gil.python();

        match &self.node {
            Literal(l) => literal_value(l),
            PreludeType(p) => Ok(format!("{:?}", p).to_object(py)),
            Rule(r) => Ok(r.name.to_object(py)),
            Unwrap(r) => Ok(r.name.to_object(py)),
            Occur(o) => Ok(o.symbol().to_object(py)),
            //KeyValue(kv) => ???
            //Range(_) => ???
            _ => Err(PyErr::new::<AttributeError, _>(())),
        }
    }

    // For KeyValue nodes, return the key/value pair.
    fn kv(&self) -> PyResult<(IVTNode, IVTNode)> {
        if let Node::KeyValue(kv) = &self.node {
            let key = IVTNode::from((*kv.key).clone());
            let value = IVTNode::from((*kv.value).clone());
            // FIXME: allow cut?
            Ok((key, value))
        } else {
            Err(PyErr::new::<AttributeError, _>(()))
        }
    }
}

// Allow access to the Debug string for any Node.
#[pyproto]
impl PyObjectProtocol for IVTNode {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", &self.node))
    }
}

#[pyproto]
impl PySequenceProtocol for IVTNode {
    // For nodes that contain a Vec<Node>, return the vector length.
    fn __len__(&self) -> PyResult<usize> {
        use Node::*;

        match &self.node {
            Map(m) => Ok(m.members.len()),
            Array(a) => Ok(a.members.len()),
            Choice(c) => Ok(c.options.len()),
            Group(g) => Ok(g.members.len()),
            _ => Err(PyErr::new::<AttributeError, _>(())),
        }
    }

    // For nodes that contain a Vec<Node>, fetch items from the vector.
    fn __getitem__(&self, idx: isize) -> PyResult<IVTNode> {
        use Node::*;

        let idx: usize = idx
            .try_into()
            .unwrap_or_else(|_| panic!("negative index unimplemented"));

        let result = match &self.node {
            Map(m) => m.members.get(idx),
            Array(a) => a.members.get(idx),
            Choice(c) => c.options.get(idx),
            Group(g) => g.members.get(idx),
            _ => {
                return Err(PyErr::new::<AttributeError, _>(()));
            }
        };
        result
            .ok_or_else(|| PyErr::new::<IndexError, _>(()))
            .map(|node| node.clone().into())
    }
}

// A Python wrapper for a set of Rules from a CDDL source text.
#[pyclass]
struct CDDLRules {
    rules_map: BTreeMap<String, Node>,
}

// Allow access to the Debug string for a CDDL rule set.
#[pyproto]
impl PyObjectProtocol for CDDLRules {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:#?}", &self.rules_map))
    }
}

// Allow access to a CDDL rule node from the rule name.
#[pyproto]
impl PyMappingProtocol for CDDLRules {
    fn __getitem__(&self, key: &str) -> PyResult<IVTNode> {
        let result = self
            .rules_map
            .get(key)
            .ok_or_else(|| PyErr::new::<IndexError, _>(()))?;
        Ok(result.clone().into())
    }
}

// Parse CDDL text and return the IVT.
#[pyfunction]
fn flatten_from_str(cddl: &str) -> PyResult<CDDLRules> {
    let result = cddl_cat::flatten::flatten_from_str(cddl).map_err(err_adapter)?;
    let result = CDDLRules { rules_map: result };
    Ok(result)
}
