mod command_parser;
mod ftp_client;

use std::io::stdin;
use crate::command_parser::{Command, determine_parameter};

fn main() {
    let ftp_address = "localhost:21";
    let username = "user";
    let password = "pass";

    if let Ok(mut client) = ftp_client::FtpClient::new(ftp_address, username, password) {
        println!("Connected to ftp server");

        loop {
            let mut input = String::new();
            let result = stdin().read_line(&mut input);

            let command = command_parser::determine_command(input.clone());

            match command {
                Command::LS => { client.list(); }
                Command::GET => {
                    let param = determine_parameter(input);
                    client.get(param.as_str());
                }
                Command::ASCII => { client.ascii_mode(); }
                Command::BINARY => { client.binary_mode(); }
                Command::QUIT => {
                    client.close();
                    break;
                }
                Command::NONE => { println!("Invalid command"); }
            }
        }
    } else {
        println!("Failed to connect to ftp server");
    }
}
