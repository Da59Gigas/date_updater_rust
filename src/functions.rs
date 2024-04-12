use std::process::{Command};

pub fn check_connection(__target: Option<&str>, __count: Option<u8>, __timeout: Option<u8>) -> bool {
    let target = __target.unwrap_or("1.1.1.1");
    let count = __count.unwrap_or(1);
    let timeout = __timeout.unwrap_or(6);
    let count_str = format!("{}", count);
    let timeout_str = format!("{}", timeout);

    let mut output2 = Command::new("ping");
    output2.arg("-c").arg(count_str).arg("-t").arg(timeout_str).arg(target);
    let nothing = output2.output().expect("Command failed to execute.");
    return match nothing.status.success() {
        true => { true }
        false => { false }
    }
}
