use clap::{Arg, Command};

fn main() {
    let name = Arg::new("name").required(true);

    let age = Arg::new("age").required(true);

    let command = Command::new("command")
        .version("0.0.1")
        .author("takokun")
        .about("feasibility study")
        .arg(name)
        .arg(age);

    match command.try_get_matches() {
        Ok(m) => {
            let n = m.value_of("name").unwrap();
            let a = m.value_of("age").unwrap().parse::<i32>().unwrap();
            println!("name {} age {}", n, a);
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}
