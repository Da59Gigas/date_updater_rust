mod functions;

//use pythonish::cmd;
use functions::check_connection;

fn main() {
    println!("Running test:");
    let _result = check_connection(None, None, None);
    println!("{}", _result);
}
