use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;
fn main() {
    println!("hit it!");
    const K_PORT: i32 = 3333;
    let connect_string = format!("localhost:{}", K_PORT);
    let mut i = 0;
    while i < 20 {
        match TcpStream::connect(&connect_string) {
            Ok(mut stream) => {

                println!("Iteration {}: Successfully connected to server on port 3333",i);
                let msg = b"hello!!";
                stream.write(msg).unwrap();
                println!("Sent hello. Waiting for reply....");
                let mut data = [0 as u8; 7]; // 6 byte buffer
                match stream.read_exact(&mut data) {
                    Ok(_) => {
                        if &data == msg {
                            println!("reply is ok!");
                        } else {
                            let text = from_utf8(&data).unwrap();
                            println!("Unexpected reply: {}", text);
                        }
                    },
                    Err(e) => {
                        println!("failed to receive data {}", e);
                    }
                }
            },
            Err(e) => {
                println!("Failed to connect: {}!", e);
            }
        }
        i +=1;
    }
    println!("Terminated.");
}
