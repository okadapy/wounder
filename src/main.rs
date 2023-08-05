use std::io;

use log::debug;

mod bodyparts;
mod wounds;

fn main() {
    pretty_env_logger::init_timed();
    let mut buf = String::new();
    let wound = bodyparts::Bodypart::generate();
    println!("{}", wound);
    debug!("Ожидаю ввод пользователя перед выходом.");
    io::stdin().read_line(&mut buf).unwrap();
}
