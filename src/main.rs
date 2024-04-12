mod functions;

use pythonish::cmd;
use functions::check_connection;

fn main() {
    let _result = cmd(String::from("ping -c 1 -t 6 1.1.1.1"));
    let _result2 = check_connection(None, None, None);
    print!("{}", _result)
}
