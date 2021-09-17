use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;
fn main() {
    println!("hit it!");
    const K_PORT: i32 = 3333;
    const K_SIZE: usize = 1024;
    let connect_string = format!("localhost:{}", K_PORT);
    let mut i = 0;
    while i < 20 {
        match TcpStream::connect(&connect_string) {
            Ok(mut stream) => {
                println!("Iteration {}: Successfully connected to server on port 3333",i);
                let mut send_string = Vec::new();
                std::write!(&mut send_string,"this is message #>{:2}<",i).unwrap();
                println!("sending {}",from_utf8(&send_string).unwrap());
                let msg = b"hello!";
                stream.write(&send_string).unwrap();
                println!("Sent {} Waiting for reply....",from_utf8(&send_string).unwrap());
                let mut data = [0 as u8; 21];
                match stream.read_exact(&mut data) {
                    Ok(_) => {
                        if &data[..] == &send_string[..] {
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
