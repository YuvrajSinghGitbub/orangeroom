pub mod room {
    /// The connection initializer
    pub mod init {
        use std::io::Read;
        use std::net::{SocketAddr, TcpListener, TcpStream};
        use std::str::{from_utf8, FromStr};

        pub fn run() {
            println!("Initializer works");
            establish_connection();
        }

        fn establish_connection() {
            let addr: SocketAddr = SocketAddr::from_str("127.0.0.1:8080").unwrap();
            let incomming_stream: TcpListener = TcpListener::bind(addr).unwrap();

            for in_stream in incomming_stream.incoming() {
                let mut in_stream = in_stream.unwrap();

                handle_connection(&mut in_stream);
            }
        }

        #[allow(unused_variables)]
        fn handle_connection(stream: &mut TcpStream) {
            // read from the reciever
            let mut buffer = [0; 512];
            let written_till = stream.read(&mut buffer).unwrap();

            // what the reciever gave
            println!(
                "recieved: {:?}",
                from_utf8(&buffer[..written_till]).unwrap()
            );
        }
    }

    /// The reciever of the connection
    pub mod reciever {
        use std::{
            io::Write,
            net::{SocketAddr, TcpStream},
            str::FromStr,
        };

        pub fn run() {
            println!("Reciever works");
            establish_connection();
        }

        fn establish_connection() {
            let addr: SocketAddr = SocketAddr::from_str("127.0.0.1:8080").unwrap();
            let mut stream: TcpStream = TcpStream::connect(addr).unwrap();

            handle_connection(&mut stream);
        }

        #[allow(unused_variables)]
        fn handle_connection(stream: &mut TcpStream) {
            stream.write(b"hello from the other side").unwrap();
        }
    }
}
