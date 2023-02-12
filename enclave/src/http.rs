use super::laplace;
use sgx_types::*;
use std::io::{self, Read, Write};
use std::net::TcpListener;
use std::string::String;
use std::string::ToString;
use std::time::{Duration, SystemTime};
use std::vec::Vec;

const EPSILON: f32 = 0.1;
const HEADER: &'static str = "HTTP/1.1 200 OK\r\nAccept: */*\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Headers: Origin, X-Requested-With, content-type, Accept\r\n\r\n";

pub struct TCPServer {
    host: String,
    port: u16,
    addr: String,
}

impl TCPServer {
    pub fn new(host: &str, port: u16) -> Self {
        TCPServer {
            host: host.to_string(),
            port: port,
            addr: format!("{}:{}", host, port),
        }
    }

    pub fn run(&self, now: SystemTime) {
        println!("{}", &"[+] TCP server started inside Enclave");

        if let Ok(listener) = TcpListener::bind(&self.addr) {
            for stream in listener.incoming() {
                match stream {
                    Ok(mut stream) => {
                        println!("{}", &"[+] Request received:\n");

                        let mut input_data = Vec::<f32>::new();
                        let mut buffer = [0; 1024];
                        stream.read(&mut buffer).unwrap();
                        let input = String::from_utf8_lossy(&buffer[..]);

                        for line in input.lines() {
                            if !(line.contains("HTTP") || line.contains(":") || line.len() < 2) {
                                if let Ok(n) = line.parse::<f32>() {
                                    input_data.push(n);
                                }
                            }
                        }

                        println!("Input data: {:?}", &input_data);

                        let output_data = input_data
                            .into_iter()
                            .map(|i| laplace::sample(i, EPSILON, now))
                            .collect::<Vec<f32>>();

                        println!("Output data: {:?}", &output_data);

                        let result = output_data
                            .into_iter()
                            .map(|i| format!("{}\n", i))
                            .collect::<String>();

                        let response = format!("{}{}", HEADER, &result);
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
}
