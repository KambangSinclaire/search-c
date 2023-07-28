mod utils;
use std::{env, process};

fn main() {
    let user_input: Vec<String> = env::args().collect();
    let input = search_c::Input::new(&user_input).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1)
    });

    println!("The query is => {}",input.text);
    println!("The path is => {}", input.file_path);

    if let Err(e) = search_c::run(input) {
        println!("Application error {}", e);
        process::exit(1);
    }
}
