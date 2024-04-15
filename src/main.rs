#![allow(unused)]
#![allow(non_snake_case)]
mod functions;

//use pythonish::cmd;
use std::env::args;
use std::fmt::format;
use std::ops::Index;
use std::process::exit;
use functions::{get_date_from_request, string_cleaner, prepare_command, add_one_hour_to_local};
use pythonish::{cmd, flush, check_root};

// Implements the logic of the program.
fn real_main_function() {
    println!("[*] Started program");
    // TODO: Implement the wait for connection logic, using functions::check_connection;
    println!("[*] Waiting for connection is disabled. [Not implemented]");
    println!("[*] Setting constants and variable like constants");

    // Probably won't be needed, it was used in the python version
    // to be a way to get the output of a terminal.
    println!("\t[*] Setting CACHE_FILE_NAME...");
    const CACHE_FILE_NAME: &str = "teste";

    // To be used to convert the date_from_request into a format I can use to set the system date.
    println!("\t[*] Setting MONTHS array...");
    let MONTHS = ["", "Jan", "Feb", "Mar", "Apr",
        "May", "Jun", "Jul", "Aug", "Sep", "Nov", "Dec"];

    println!("\t[*] Setting ORDER_DATE_REQUEST array...");
    let ORDER_DATE_REQUEST = ["week_day", "month_day", "month", "year", "time", "gomo"];

    println!("\t[*] Setting UPDATE...");
    // Simple way to just check if the program is working, just provide at least one argument
    // It won't set anything, just show what it would set to
    println!("\t\t[*] Getting arguments...");
    let argv2: Vec<_> = args().collect();
    println!("\t\t[*] Matching possible number of arguments...");
    let DEBUG_MODE_DEACTIVATED: bool = match argv2.len() {
        0..=1 => {true},
        _ => {false},
    };
    println!("\t\t\t[*] Detected {} extra terminal arguments, DEBUG_MODE_DEACTIVATED set to {}", argv2.len()-1, DEBUG_MODE_DEACTIVATED);

    println!("\t\t[*] Checking for root permissions...");
    if !check_root() {
        println!("\t\t\t[*] Root privileges not detected.");
        match DEBUG_MODE_DEACTIVATED {
            true => {
                println!("\t\t\t[*] Program must be ran as root.");
                exit(-1);
            }
            false => {
                println!("\t\t\t[*] Root privileges needed overwritten.");
            }
        }
    }

    println!("[*] Starting main logic...");
    // Gets the current time from the metadata present in the heathers of an HTTP response.
    // This will be used to set the system time.
    println!("\t[*] Getting request and date from it...");
    let uncleaned_date: String = get_date_from_request(None);
    println!("\t[*] Cleaning received date_string...");
    let date_from_request: String = string_cleaner(uncleaned_date);
    println!("\t[*] Dissecting date_string...");
 	let split_ed: Vec<_> = date_from_request.split(' ').collect();
    let index1 = ORDER_DATE_REQUEST.iter().position(|&x| x == "year").unwrap();
    let year = split_ed[index1];
    print!("\t\tYear: {}", year);
    let index2 = ORDER_DATE_REQUEST.iter().position(|&x| x == "month").unwrap();
    let month = MONTHS.iter().position(|&x| x == split_ed[index2]).unwrap();
    print!(" | Month: {}", month);
    let index3 = ORDER_DATE_REQUEST.iter().position(|&x| x == "month_day").unwrap();
    let day = split_ed[index3];
    print!(" | Day: {}", day);
    let index4 = ORDER_DATE_REQUEST.iter().position(|&x| x == "time").unwrap();
    let time = split_ed[index4];
    print!(" | Time: {}", time);
    let index5 = ORDER_DATE_REQUEST.iter().position(|&x| x == "gomo").unwrap();
    let gomo = split_ed[index5];
    print!(" | Gomo: {}\n", gomo);

    // 	'sudo date --set="2024-1-2 13:50:10"'
    println!("\t[*] Starting the analise of the date to set...");
    println!("\t\t[*] Checking the received GOMO...");
    let	command_to_run = prepare_command(year, month, day, time, gomo);
    println!("\t\t\t[*] Command to run: [{command_to_run}]");
    println!("\t[*] Starting the execution of the set date command...");
    match DEBUG_MODE_DEACTIVATED {
        true => {
            let output = cmd(command_to_run);
            println!("{}", output)
        }
        false => {
            println!("\t\t[*] Command execution aborted, UPDATE is False.")
        }
    }

    println!("\t[*] Starting the execution of the SUMMER TIME logic...");
    println!("\t\t[*] Checking if the system is in summer time...");
    let current_sys_date = cmd(String::from("date")).replace("\n", "");
    println!("\t\t\t[*] Current system date/time: ({})", current_sys_date);
    match current_sys_date.contains("BST") {
        true => {
            println!("\t\t\t[*] System is in Summer Time mode.");
            println!("\t\t[*] Initiating the 'Add one hour process'...");
            println!("\t\t\t[*] Creating new date string...");
            let new_date_to_set = add_one_hour_to_local(current_sys_date,
                                                        MONTHS);
            println!("\t\t\t[*] New date: ({})", new_date_to_set);
            println!("\t\t[*] Starting the execution of the set summer date command...");
            match DEBUG_MODE_DEACTIVATED {
                true => {
                    let command_to_run2= format!("sudo date --set='{}'", new_date_to_set);
                    println!("\t\t\t[*] Command to run: [{command_to_run2}]");
                    println!("\t\t\t[*] Running...");
                    let output = cmd(command_to_run2);
                    println!("{}", output)
                }
                false => {println!("\t\t[*] Command execution aborted, UPDATE is False.") }
            }
        }
        false => {println!("\t\t\t[*] System is not in Summer Time mode.")}
    }
    println!("[*] Ended execution of the program.")
}

// Just to run whatever I need, be it tests or the program itself.
fn main() {
    real_main_function()
}
