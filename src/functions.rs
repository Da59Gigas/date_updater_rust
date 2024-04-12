use std::process::{Command};

pub fn check_connection(__target: Option<&str>, __count: Option<u8>, __timeout: Option<u8>) -> bool {
    let target = __target.unwrap_or("1.1.1.1");
    let count = __count.unwrap_or(1);
    let timeout = __timeout.unwrap_or(6);
    // python: command = f"ping -c {count} -t {timeout} {target}"
    // try to figure out how to either make the equivalent of an fstring
    // or convert u8 to String or &str
    let mut output = Command::new("ping").arg("-c").arg(count)
        .arg("-t").arg(timeout).arg(target);
    let nothing = output.output().expect("Command failed to execute.");
    return match nothing.status.success() {
        true => { true }
        false => { false }
    }
}
