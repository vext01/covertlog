#![feature(lazy_cell)]

use std::{
    fmt::Debug,
    sync::{LazyLock, Mutex},
    thread,
};

static LOG: LazyLock<Mutex<Vec<String>>> = LazyLock::new(|| Mutex::new(Vec::new()));

/// Push a string into the buffered log.
pub fn push_str(msg: &str) {
    let s = format!("{:?}: {}", thread::current().id(), msg);
    LOG.lock().unwrap().push(s);
}

/// Push `thing`'s `Debug` string into the buffered log.
pub fn push_dbg(thing: &dyn Debug) {
    push_str(&format!("{:?}", thing));
}

/// Emit (to stderr) and clear the buffered log.
pub fn flush() {
    let mut v = LOG.lock().unwrap();
    eprintln!("{}", v.join("\n"));
    v.clear();
}
