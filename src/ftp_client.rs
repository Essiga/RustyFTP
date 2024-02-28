use std::io::{Read, Write};
use std::net::TcpStream;
use std::{io, str};
use std::fs::File;

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

        let mut data_stream = self.open_data_stream()?;

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

        self.close_data_stream(&mut data_stream)?;
        Ok(())
    }

    pub (crate) fn ascii_mode(&mut self) -> std::io::Result<()> {
        let mut response = String::new();
        send_command(&mut self.control_stream, "TYPE A\r\n")?;
        read_response(&mut self.control_stream, &mut response)?;
        #[cfg(debug_assertions)]
        println!("Server response: {}", response);
        Ok(())
    }

    pub (crate) fn binary_mode(&mut self) -> std::io::Result<()> {
        let mut response = String::new();
        send_command(&mut self.control_stream, "TYPE I\r\n")?;
        read_response(&mut self.control_stream, &mut response)?;
        #[cfg(debug_assertions)]
        println!("Server response: {}", response);
        Ok(())
    }

    pub (crate) fn get(&mut self, filename: &str) -> std::io::Result<()> {
        let mut response = String::new();

        let mut data_stream = self.open_data_stream()?;

        send_command(&mut self.control_stream, &format!("RETR {}\r\n", filename))?;
        read_response(&mut self.control_stream, &mut response)?;

        if response.starts_with("550") {
            println!("File not found");
            return Ok(());
        }

        #[cfg(debug_assertions)]
        println!("Server response: {}", response);

        let mut response_data: Vec<u8> = Vec::new();

        while let Ok(bytes_read) = read_response_bytes(&mut data_stream, &mut response_data) {
            if bytes_read == 0 {
                break;
            }
        }

        read_response(&mut self.control_stream, &mut response)?;
        println!("Server response: {}", response);

        let mut file = File::create(filename)?;
        file.write_all(&*response_data).expect("error writing to file");
        println!("File saved!");

        self.close_data_stream(&mut data_stream)?;

        Ok(())
    }

    fn open_data_stream(&mut self) -> std::io::Result<TcpStream> {
        let mut response = String::new();

        send_command(&mut self.control_stream, "EPSV\r\n")?; //using epsv instead of pasv because it's newer and allows for ipv6
        read_response(&mut self.control_stream, &mut response)?;
        #[cfg(debug_assertions)]
        println!("Server response: {}", response);
        let port = parse_port(&response);

        #[cfg(debug_assertions)]
        println!("Connecting to data port: {}", port);
        TcpStream::connect(format!("localhost:{}", port))
    }

    fn close_data_stream(&mut self, data_stream: &mut TcpStream) -> std::io::Result<()> {
        data_stream.shutdown(std::net::Shutdown::Both)?;
        Ok(())
    }

    pub (crate) fn close(&mut self) {
        println!("Closing session!");
        send_command(&mut self.control_stream, "QUIT\r\n").unwrap();
        self.control_stream.shutdown(std::net::Shutdown::Both).unwrap();
    }
}

fn read_response(stream: &mut TcpStream, response: &mut String) -> std::io::Result<usize> {
    let mut buffer = [0; 4096];
    let bytes_read = stream.read(&mut buffer)?;
    *response = str::from_utf8(&buffer[..bytes_read])
        .unwrap_or_default()
        .to_string();
    Ok(bytes_read)
}

fn read_response_bytes(stream: &mut TcpStream, response: &mut Vec<u8>) -> std::io::Result<usize> {
    let mut buffer = [0; 4096];
    let bytes_read = stream.read(&mut buffer)?;
    response.append(&mut buffer[.. bytes_read].to_vec());
    Ok(bytes_read)
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

fn parse_ftp_response(response: &str) -> (i32, String) {
    let mut parts: Vec<&str> = response.splitn(2, ' ').collect();
    let code = parts[0].parse().unwrap();
    let message = parts[1].to_string();
    (code, message)
}