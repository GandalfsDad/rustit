use std::env;
use rustit::cli::parser::parse_command;



fn main() {
    let args: Vec<String> = env::args().collect();
    parse_command(&args)
}
