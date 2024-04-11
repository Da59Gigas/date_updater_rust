use pythonish::cmd;

fn main() {
    let teste = cmd(String::from("systemctl status updat"));
    print!("{}", teste)
}
