mod command_parser;
mod ftp_client;

use std::io::stdin;

fn main() {
    let ftp_address = "localhost:21";
    let username = "user";
    let password = "pass";

    let mut client = ftp_client::FtpClient::new(ftp_address, username, password);


    let mut input = String::new();
    let b1 = stdin().read_line(&mut input);

    let command = command_parser::determine_command(input);
    println!("{:?}", command);


    // if command == command_parser::Command::LS {
    //     //ftp_client::ls();
    // } else if(command == command_parser::Command::GET) {
    //     // ftp_client::get();
    // } else if(command == command_parser::Command::MGET) {
    //     // ftp_client::mget();
    // } else {
    //     println!("Invalid command");
    // }
}
