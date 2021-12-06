use std::io::{self, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;

const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 32;


fn main() {
    let mut client = TcpStream::connect(LOCAL).expect("Stream failed to connect");
    server.set_nonblocking(true).expect("failed to initialize non-blocking");

    let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || loop {
        let mut buff = vec![o; MSG_SIZE];

        match client.read_exact(&mut buff) {
            Ok(_) => {
                let msg = buff.into_iter().take_while(|&x| x!=0).collect::<Vec<_>>();
                println!("message recieved {:?}", msg);
            }
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
            Err(_) => {
                println!("connections with server was severed");
                break;
            }
        }


    })
}
