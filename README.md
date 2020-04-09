
## `cddlcat` for Python

This builds a python module granting access to the [`cddl-cat`] crate.

To "install", copy `target/release/libcddlcat.so` to some place in your python `sys.path`, and rename it to `cddlcat.so`.

TODO list:
- Add help to python objects
- Make ValidateError visible so that "except ValidateError" will work.
- Grant access to more `cddl-cat` functionality:
  - `flatten_from_str`
  - `BasicContext::new`
  - `Context.rules.get(rule_name)`

[`cddl-cat`]: https://crates.io/crates/cddl-cat
