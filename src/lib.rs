pub mod room {
    /// The connection initializer
    pub mod init {
        use std::{
            io::{Read, Write},
            net::TcpListener,
        };

        pub fn run() {
            let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
            println!("Server listening on port 8080...");

            for stream in listener.incoming() {
                let mut stream = stream.unwrap();
                println!("New client connected!");

                loop {
                    let mut buffer = [0; 1024];
                    let bytes_read = stream.read(&mut buffer).unwrap();
                    let message = String::from_utf8_lossy(&buffer[..bytes_read]);

                    if message.trim() == "exit" {
                        println!("Client disconnected!");
                        break;
                    }

                    println!("Received message: {}", message.trim());

                    stream.write_all(message.as_bytes()).unwrap();
                }
            }
        }
    }

    /// The reciever of the connection
    pub mod reciever {
        use std::{
            io::{self, BufRead, Read, Write},
            net::TcpStream,
        };

        pub fn run() {
            let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();
            println!("Connected to server!");

            let stdin = io::stdin();
            loop {
                let mut buffer = String::new();
                stdin.lock().read_line(&mut buffer).unwrap();

                stream.write_all(buffer.as_bytes()).unwrap();

                let mut response_buffer = [0; 1024];
                let bytes_read = stream.read(&mut response_buffer).unwrap();
                let response = String::from_utf8_lossy(&response_buffer[..bytes_read]);
                println!("Received response: {}", response.trim());

                if buffer.trim() == "exit" {
                    break;
                }
            }
        }
    }
}
