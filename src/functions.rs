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
    let buffer1: Vec<_> = output.split("ate: ").collect();
    let buffer2: Vec<_> = buffer1[1].split("\n").collect();
    let date_from_request: &str = buffer2[0];
    return String::from(date_from_request);
}

pub fn string_cleaner(string_to_clean: String) -> String {
    let mut string_cleaned: String = string_to_clean;
    string_cleaned = String::from(string_cleaned.trim());
    string_cleaned = String::from(string_cleaned.replace("\n", ""));
    let to_return_so_it_is_not_a_mutable_string = string_cleaned;
    return to_return_so_it_is_not_a_mutable_string;
}

pub fn prepare_command(year: &str, month: usize, day: &str,
                       time: &str, gomo: &str) -> String {
    if gomo == "GMT" {
        println!("\t\t[*] Recognized GOMO [GMT]");
        println!("\t\t[*] Creating the string for the command to be executed to set the system date.");
        return format!("sudo date --set='{year}-{month}-{day} {time}'");
    } else {
        panic!("Time frame not recognized [{gomo}]");
    }
}

pub fn add_one_hour_to_local(current_system_date: String, months_constant: [&str; 12]) -> String {
    // current_system_date = "Sun Apr 14 11:22:20 PM BST 2024"
    // current_time_vector = vec!["Sun", "Apr", "14", "11:22:20", "PM", "BST", "2024"]
    let current_time_vector: Vec<_> = current_system_date.split(" ").collect();

    let month = months_constant.iter().position(|&x| x == current_time_vector[1]).unwrap();

    let all_times: Vec<_> = current_time_vector[3].split(":").collect();
    let hour: u8 = all_times[0].parse().unwrap();
    let minute = all_times[1];
    let second = all_times[2];

    let time: String = format!("{}:{}:{}", hour+1, minute, second);
    let command_ = String::from(format!("sudo date --set='{}-{}-{} {} {} {}'",
                                        current_time_vector[6], month, current_time_vector[2],
                                        time, current_time_vector[4], current_time_vector[5]));

    return command_;
}
