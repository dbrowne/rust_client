use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;
use chrono::{DateTime, Utc, Duration};

fn main() {
    const K_PORT: i32 = 3333;
    const K_HOST: &str = "localhost";
    const K_BUFSIZ:usize = 1024;
    const K_ITERATIONS: i32 = 10;
    let connect_string = format!("{}:{}",K_HOST ,K_PORT);
    let mut ctr = 0;
    while ctr < K_ITERATIONS {
        match TcpStream::connect(&connect_string) {
            Ok(mut stream) => {
                println!("{}: iteration {}: Successfully connected to server on port 3333",Utc::now(), ctr);
                let mut send_string = Vec::new();
                std::write!(&mut send_string,"{}: v this is message #>{:25}<",Utc::now(),   ctr).unwrap();
                println!("{}: sending {}",Utc::now(),from_utf8(&send_string).unwrap());
                match stream.write(&send_string){
                    Ok(n) => {
                        println!("{}: write ok! {} bytes",Utc::now(),n);
                    },
                    Err(e) =>{
                        println!("{}: Canna write the data Captn'! {}",Utc::now(), e);
                    }
                }

                match stream.flush(){
                    Ok(_) =>{},
                    Err(e) =>{
                        println!("{}: Canna flush it {}",Utc::now(), e);
                    }
                }


                println!("{}: Sent {} Waiting for reply....",Utc::now(), from_utf8(&send_string).unwrap());
                let mut data = [0 as u8; K_BUFSIZ];
                match stream.read(&mut data) {
                    Ok(n) => {
                        if &data[..n] == &send_string[..] {
                            println!("{}: reply is ok! read {} bytes",Utc::now(), n);
                        } else {
                            let text = from_utf8(&data).unwrap();
                            println!("{}: Unexpected reply: {}",Utc::now(), text);
                        }
                    },
                    Err(e) => {
                        println!("{}: failed to receive data {}",Utc::now(), e);
                    }
                }
            },
            Err(e) => {
                println!("{}: Failed to connect: {}!",Utc::now(), e);
            }
        }
        ctr +=1;
    }
    println!("{}: Terminated.",Utc::now());
}
