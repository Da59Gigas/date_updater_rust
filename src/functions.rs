#![allow(unused)]
use std::process::{Command};
use pythonish::cmd;

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
        true => { true },
        // Some fake falses still come up sometimes.
        false => { false }
    }
}

pub fn get_date_from_request(url: Option<String>) -> String {
    let url = url.unwrap_or(String::from("https://duckduckgo.com/"));
    let command = format!("curl --verbose --insecure --silent  -i {}", url);
    let output = cmd(command);

    //TODO: Think of a way to extract date from output.
    let teste: Vec<_> = output.split("Date").collect();
    print!("{}", teste.len());

    return String::from("None");
}
