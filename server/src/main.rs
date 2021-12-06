use std::io::{ErrorKind, Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;


const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 32;


fn main() {
    let server = TcpListener::bind(LOCAL).expect("Listener failed to bind");
    server.set_nonblocking(true).expect("failed to initialize non-blocking");

    let mut clients = vec![];

    let (tx,rx) = mpsc::channel::<String>();
    loop {
        if let Ok((mut socket, addr)) = server.accept() {
            println!("Client {} connected", addr);
            let tx = tx.clone();
            clients.push(socket.try_clone().expect("failed to clone client"));

            thread::spawn(move || loop {
                let mut buff = vec![0; MSG_SIZE];

                match socket.read_exact(&mut buff) {
                    Ok(_) => {
                        let msg = buff.into_iter().take_while(|&x| x!=0).collect::<Vec<_>>();
                        let msg = String::from_utf8(msg).expect("Invalid utf8 message");

                    }
                    Err(_) => {}
                }


            })


        }
    }


}