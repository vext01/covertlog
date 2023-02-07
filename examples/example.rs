use covertlog;
use std::process::Command;

fn main() {
    // You can log formatted strings like this:
    covertlog::push_str(&format!("I got here"));

    // Or the debug string of somrthing like this:
    let cmd = Command::new("/bin/ls");
    covertlog::push_dbg(&cmd);

    // When you are ready to print the log, do:
    covertlog::flush();
}
