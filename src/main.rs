#![allow(unused)]
#![allow(non_snake_case)]
mod functions;

//use pythonish::cmd;
use std::env::args;
use std::ops::Index;
use std::process::exit;
use functions::{get_date_from_request, string_cleaner, prepare_command};
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
}

// Just to run whatever I need, be it tests or the program itself.
fn main() {
    real_main_function()
}

// TODO: python logic for the rest of the program

// 	if gomo == 'GMT':
// 		print('Time frame recognized [GMT]')
// 		if UPDATE:
// 			print('Preparing to set date..')
// 			cmd(command=command_)
// 		else:
// 			print('UPDATE = False')
// 	else:
// 		print(f'Time frame not recognized [{gomo}]')
// 	__seted_date = get_cmd("date").replace('\n', '').split(' ')
// 	print(__seted_date)
// 	if "BST" in __seted_date:
// 		print('Summer time detected')
// 		time2 = __seted_date[3].split(':')
// 		if UPDATE:
// 			command_ = f'sudo date --set="{__seted_date[-1]}-{months.index(__seted_date[1])}-{__seted_date[2]} {int(time2[0])+1}:{time2[1]}:{time2[2]} {__seted_date[4]} {__seted_date[5]}"'
// 			print(f"command_ = [{command_}]")
// 			cmd(command=command_)
// 		else:
// 			print('UPDATE = False')
