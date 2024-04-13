#![allow(unused)]
#![allow(non_snake_case)]
mod functions;

//use pythonish::cmd;
use std::env::args;
use functions::{get_date_from_request, string_cleaner};
use pythonish::flush;

// Implements the logic of the program.
fn real_main_function() {
    println!("[*] Started program");
    // TODO: Implement the wait for connection logic, using functions::check_connection;
    println!("[*] Waiting for connection is not implemented yet.");
    // Probably won't be needed, it was used in the python version
    // to be a way to get the output of a terminal.
    println!("[*] Setting constants and variable like constants");

    println!("[*] Setting CACHE_FILE_NAME...");
    const CACHE_FILE_NAME: &str = "teste";

    // To be used to convert the date_from_request into a format I can use to set the system date.
    println!("[*] Setting MONTHS array...");
    let MONTHS = ["", "Jan", "Feb", "Mar", "Apr",
        "May", "Jun", "Jul", "Aug", "Sep", "Nov", "Dec"];

    println!("[*] Setting ORDER_DATE_REQUEST array...");
    let ORDER_DATE_REQUEST = ["week_day", "month_day", "month", "year", "time", "gomo"];

    println!("[*] Setting UPDATE...");
    // Simple way to just check if the program is working, just provide at least one argument
    // It won't set anything, just show what it would set to
    println!("\t[*] Getting arguments...");
    let argv2: Vec<_> = args().collect();
    println!("\t[*] Matching possible number of arguments...");
    let UPDATE: bool = match argv2.len() {
        0..=1 => {true},
        _ => {false},
    };

    println!("[*] Starting main logic...");
    // Gets the current time from the metadata present in the heathers of an HTTP response.
    // This will be used to set the system time.
    println!("\t[*] Getting request and date from it...");
    let uncleaned_date: String = get_date_from_request(None);
    println!("\t[*] Cleaning received date_string...");
    let date_from_request: String = string_cleaner(uncleaned_date);
    print!("{}", date_from_request)
}

// Just to run whatever I need, be it tests or the program itself.
fn main() {
    //real_main_function()
    let string = String::from("      Sat, 13 Apr 2024 17:05:48\n GMT\n           ");
    print!("{}", string_cleaner(string))
}

// TODO: python logic for parsing the date

// 	date_from_request
// 	split_ed = date_from_request.split(sep=' ')
// 	year = split_ed[values.index('year')]
// 	month = months.index(split_ed[values.index('month')])
// 	day = split_ed[values.index('month_day')]
// 	time = split_ed[values.index('time')]
// 	gomo = split_ed[values.index('gomo')]

// TODO: python logic for the rest of the program

// 	# sudo date --set="2024-1-2 13:50:10"
// 	command_ = f'sudo date --set="{year}-{month}-{day} {time}"'
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