mod command_parser;
mod ftp_client;

use std::io::stdin;
use crate::command_parser::Command;

fn main() {
    let ftp_address = "localhost:21";
    let username = "user";
    let password = "pass";

    if let Ok(mut client) = ftp_client::FtpClient::new(ftp_address, username, password) {
        println!("Connected to ftp server");

        loop {
            let mut input = String::new();
            let b1 = stdin().read_line(&mut input);

            let command = command_parser::determine_command(input);
            println!("{:?}", command);

            match command {
                Command::LS => {client.list();}
                Command::GET => {}
                Command::MGET => {}
                Command::NONE => {println!("Invalid command");}
            }
        }
    } else {
        println!("Failed to connect to ftp server");
    }



    // let mut input = String::new();
    // let b1 = stdin().read_line(&mut input);
    //
    // let command = command_parser::determine_command(input);
    // println!("{:?}", command);


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
