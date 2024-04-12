#![allow(unused)]
mod functions;

//use pythonish::cmd;
use std::env::args;
use functions::get_request;


fn real_main_function() {
    const CACHE_FILE_NAME: &str = "teste";

    let argv2: Vec<_> = args().collect();
    match argv2.len() {
        0..=1 => {const UPDATE: bool = true;},
        _ => {const UPDATE: bool = false;},
    };
}


fn main() {
    get_request(None);
}
