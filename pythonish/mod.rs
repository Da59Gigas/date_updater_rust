#![allow(unused)]
use std::io::{Write, stdin, stdout, ErrorKind, BufReader, BufRead, Read};
use std::fs::File;
use std::process::{Command, exit};
use std::env::{var, VarError};

pub mod files;

pub fn input(msg: &str) -> String {
    print!("{}", msg);
    let mut name: String = String::new();
    flush();
    stdin().read_line(&mut name).expect("Didn't Receive Input");
    return name
}

pub fn flush() {
    let _ = stdout().flush();
}

pub fn cmd(command: String) -> String {
    let mut cutted: Vec<&str> = command.split(" ").collect();
    let mut output = Command::new(cutted[0]);
    for string in &mut cutted[1..] {
        output.arg(string);
    }
    let nothing = output.output().expect("Command failed to execute.");
    return String::from(String::from_utf8_lossy(&nothing.stdout));
}

pub fn check_root() -> bool {
    match var("USER") {
        Ok(name) => {
            if name != "root" {
                return false;
            };
        },
        Err(error) => {
            panic!("Something went wrong when checking for username: {}", error);
        }
    };
    return true;
}
