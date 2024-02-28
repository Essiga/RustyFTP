use std::io::{Read, Write};
use std::net::TcpStream;
use std::{io, str};

pub(crate) struct FtpClient{
    control_stream : TcpStream,
}

impl FtpClient {

    pub fn new(addr: &str, username: &str, password: &str) -> io::Result<Self> {
        println!("Connecting to {}", addr);

        let mut control_stream = TcpStream::connect(addr)?;
        let mut response = String::new();

        read_response(&mut control_stream, &mut response)?;
        #[cfg(debug_assertions)]
        println!("Server response: {}", response);

        send_command(&mut control_stream, &format!("USER {}\r\n", username))?;
        read_response(&mut control_stream, &mut response)?;
        #[cfg(debug_assertions)]
        println!("Server response: {}", response);

        send_command(&mut control_stream, &format!("PASS {}\r\n", password))?;
        read_response(&mut control_stream, &mut response)?;
        #[cfg(debug_assertions)]
        println!("Server response: {}", response);
        #[cfg(debug_assertions)]
        println!("Connected to {}", addr);

        Ok(Self { control_stream })
    }

    pub (crate) fn list(&mut self) -> std::io::Result<()> {
        println!("LIST COMMAND:");
        let mut response = String::new();

        send_command(&mut self.control_stream, "EPSV\r\n")?; //using epsv instead of pasv because it's newer and allows for ipv6
        read_response(&mut self.control_stream, &mut response)?;
        #[cfg(debug_assertions)]
        println!("Server response: {}", response);
        let port = parse_port(&response);

        #[cfg(debug_assertions)]
        println!("Connecting to data port: {}", port);
        let mut data_stream = TcpStream::connect(format!("localhost:{}", port))?;

        #[cfg(debug_assertions)]
        println!("Sending LIST command");
        send_command(&mut self.control_stream, "LIST\r\n")?;

        read_response(&mut self.control_stream, &mut response)?;
        #[cfg(debug_assertions)]
        println!("Server response: {}", response);

        read_response(&mut data_stream, &mut response)?;
        println!("{}", response);

        read_response(&mut self.control_stream, &mut response)?;
        #[cfg(debug_assertions)]
        println!("Server response: {}", response);


        data_stream.shutdown(std::net::Shutdown::Both)?;
        Ok(())
    }
}

fn read_response(stream: &mut TcpStream, response: &mut String) -> std::io::Result<()> {
    let mut buffer = [0; 512];
    let bytes_read = stream.read(&mut buffer)?;
    *response = str::from_utf8(&buffer[..bytes_read])
        .unwrap_or_default()
        .to_string();
    Ok(())
}

fn send_command(stream: &mut TcpStream, command: &str) -> std::io::Result<()> {
    stream.write_all(command.as_bytes())?;
    Ok(())
}

fn parse_port(response : &str) -> String{
    let response_without_linebreaks = remove_line_breaks(response);
    let mut parts: Vec<&str> = response_without_linebreaks.split("|||").collect();

    let parts = parts[1].split("|").collect::<Vec<&str>>();
    #[cfg(debug_assertions)]
    println!("{:?}", parts[0]);
    return parts[0].to_string();
}

fn remove_line_breaks(input: &str) -> String {
    return input.replace("\r\n", "");
}
