#![allow(unused)]
#![allow(non_snake_case)]
mod functions;

//use pythonish::cmd;
use std::env::args;
use functions::get_date_from_request;

// Implements the logic of the program.
fn real_main_function() {
    println!("[*] Started program");
    // TODO: Implement the wait for connection logic, using functions::check_connection;
    println!("[*] Waiting for connection is not implemented yet.");
    // Probably won't be needed, it was used in the python version
    // to be a way to get the output of a terminal.
    const CACHE_FILE_NAME: &str = "teste";

    // Simple way to just check if the program is working, just provide at least one argument
    let argv2: Vec<_> = args().collect();
    let UPDATE: bool = match argv2.len() {
        0..=1 => {true},
        _ => {false},
    };
    // Gets the current time from the metadata present in the heathers of an HTTP response.
    // This will be used to set the system time.
    let date_from_request: String = get_date_from_request(None);

    // To be used to convert the date_from_request into a format I can use to set the system date.
    let months = ["", "Jan", "Feb", "Mar", "Apr",
        "May", "Jun", "Jul", "Aug", "Sep", "Nov", "Dec"];
    let values = ["week_day", "month_day", "month", "year", "time", "gomo"];
}

// Just to run whatever I need, be it tests or the program itself.
fn main() {

}

// python logic

// 	to_parse = read_request().replace('\n', ' ').strip()
// 	months = ['', 'Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul', 'Aug', 'Sep', 'Nov', 'Dec']
// 	values = ('week_day', 'month_day', 'month', 'year', 'time', 'gomo')
// 	split_ed = to_parse.split(sep=' ')
// 	year = split_ed[values.index('year')]
// 	month = months.index(split_ed[values.index('month')])
// 	day = split_ed[values.index('month_day')]
// 	time = split_ed[values.index('time')]
// 	gomo = split_ed[values.index('gomo')]
//
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