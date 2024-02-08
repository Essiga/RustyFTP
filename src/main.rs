mod command_parser;
mod ftp_client;

use std::io::stdin;

fn main() {
    let mut input = String::new();
    let b1 = stdin().read_line(&mut input);


    let command = command_parser::determine_command(input);
    println!("{:?}", command);
}
