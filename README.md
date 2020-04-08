
## `cddlcat` for Python

This builds a python module granting access to the [`cddl-cat`] crate.

The nightly version of rust is required.  This is because `pyo3` requires it.

To "install", copy `target/release/libcddlcat.so` to some place in your python `sys.path`, and rename it to `cddlcat.so`.

TODO list:
- Maybe I should try `rust-cypthon` instead of `pyo3`?  That would solve the dependency on nightly.
- Implement python exceptions
- Add help to python objects
- Grant access to more `cddl-cat` functionality:
  - `flatten_from_str`
  - `BasicContext::new`
  - `Context.rules.get(rule_name)`

[`cddl-cat`]: https://crates.io/crates/cddl-cat
