use std::io::{Write, stdin, stdout, ErrorKind, BufReader, BufRead, Read};
use std::fs::File;

pub fn open_file(file_name: &str, __mode: Option<&str>, __replace: Option<bool>) -> File {
    let mode = __mode.unwrap_or("rt");
    let replace = __replace.unwrap_or(false);
    if mode.contains("+") && (!(file_exists(file_name)) | replace){
        let output = File::create(file_name);
        let mut opened_file = match output {
            Ok(file) => { file },
            Err(error) => { panic!("Problem creating '{}' file. Error{:?}", file_name, error) }
        };
        return opened_file;
    }
    if mode.contains("r") | mode.contains("w") {
        let content = File::open(file_name);
        let mut opened_file = match content {
            Ok(file) => file,
            Err(error1) => match error1.kind() {
                ErrorKind::NotFound => panic!("File {} not found.", file_name),
                _other_errors => panic!("Problem creating '{}' file. Error{:?}",
                                       file_name, _other_errors)}
        };
        return opened_file;
    } else { panic!("Mode not recognised [{}].", mode) }
}

pub fn file_exists(file_name: &str) -> bool {
    let content = File::open(file_name);
    let opened_file = match content {
        Ok(file) => return true,
        Err(error1) => match error1.kind() {
            ErrorKind::NotFound => return false,
            _other_errors => panic!("Problem creating '{}' file. Error{:?}",
                                    file_name, _other_errors) }
    };
}

pub fn read_file_str(mut file: File) -> String {
    let mut contents_str = String::new();
    file.read_to_string(&mut contents_str);
    return contents_str
}

pub fn str_of_file_to_vector(str_to_cut: String) -> Vec<String> {
    let mut contents_list: Vec<String> = Vec::new();
    let mut contents_str = String::new();
    for line in contents_str.lines() {
        contents_list.append(&mut vec![line.to_string()])
    }
    return contents_list
}
