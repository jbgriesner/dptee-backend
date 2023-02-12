use sgx_types::*;
use std::io::{self, Read, Write};
use std::net::TcpListener;
use std::string::String;
use std::string::ToString;
use std::vec::Vec;

const HEADER: &'static str = "HTTP/1.1 200 OK\r\nAccept: */*\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Headers: Origin, X-Requested-With, content-type, Accept\r\n\r\n";

pub struct TCPServer {
    host: String,
    port: u16,
}

impl TCPServer {
    pub fn new(host: &str, port: u16) -> Self {
        TCPServer {
            host: host.to_string(),
            port: port,
        }
    }

    pub fn run(&self) {
        let mut input_data = Vec::<f32>::new();
        let addr = format!("{}:{}", self.host, self.port);
        let listener = TcpListener::bind(&addr).unwrap();
        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let mut buffer = [0; 1024];
                    stream.read(&mut buffer).unwrap();
                    let input = String::from_utf8_lossy(&buffer[..]);

                    println!("Request received: {}", &input);

                    for line in input.lines() {
                        if !(line.contains("HTTP") || line.contains(":") || line.len() < 2) {
                            if let Ok(n) = line.parse::<f32>() {
                                input_data.push(n);
                            }
                        }
                    }

                    println!("Data: {:?}", &input_data);

                    let result: String = input_data
                        .into_iter()
                        .map(|i| format!("{}\n", i))
                        .collect::<String>();

                    // let response = "HTTP/1.1 200 OK\r\nAccept: */*\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Headers: Origin, X-Requested-With, content-type, Accept\r\n\r\nHello received!";
                    // let response_pre = "HTTP/1.1 200 OK\r\nAccept: */*\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Headers: Origin, X-Requested-With, content-type, Accept\r\n\r\n";
                    let response = format!("{}:{:?}", HEADER, &result);

                    stream.write(response.as_bytes()).unwrap();
                    stream.flush().unwrap();
                }
                Err(e) => {
                    println!("Error on accept {}", e);
                }
            }
        }
    }
}
