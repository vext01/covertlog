# covertlog

`covertlog` is a (very) simple Rust crate that allows you to "buffer up"
logging lines, emitting them (to stderr) at a later time.

This is useful for debugging inside test suites where the very act of logging
to stderr (or stdout) would cause (otherwise passing) tests to fail.

For example, the [`lang_tester`](https://crates.io/crates/lang_tester) crate
can be used to fuzzy match the output of running some program under test, and
adding logging to stderr/atdout could cause maching to spuriously fail.

`covertlog` can be used to "buffer up" log messages without immediately emitting
them, only flushing them out when the code reaches a strategically positioned
call to `covertlog::flush()`.
