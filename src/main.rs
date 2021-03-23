use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 50]; // using 50 byte buffer
    // Read data to the buffer
    while match stream.read(&mut data) {
        // Data read successfully
        Ok(size) => {
            println!("Received data size is: {}", size-1);
            if size == 0 {
                println!("Client seems disconnected, no need to continue listening");
                stream.shutdown(Shutdown::Both).unwrap();
                false
            } else {
                // echo everything!
                stream.write(&data[0..size]).unwrap();
                true
            }
        },
        // Error process
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() {
    // TcpListener is an object listen to the port
    let listener = TcpListener::bind("127.0.0.1:3333").unwrap();
    println!("Server listening on port 3333");
    // When there is a new connection, listener.incoming will return it as a stream
    for stream in listener.incoming() {
        println!("There is a new incoming connection.");
        match stream {
            Ok(stream) => {
                // The connection stream is valid, then print the client's address and port
                println!("New connection: {}", stream.peer_addr().unwrap());
                // Start a thread to handle the interaction
                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => {
                // If there is anything wrong, print an error
                println!("Error: {}", e);
            }
        }
    }
    // close the socket server
    print!("xxxxxxxxx About to drop the listener xxxxxxxxx");
    drop(listener);
}
