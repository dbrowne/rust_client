use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;
use chrono::Utc;
use std::{thread, time};
use structopt::StructOpt;

fn main() {
    const K_PORT: &str = "3333";
    const K_HOST: &str = "localhost";
    const K_BUFSIZ: usize = 1024;
    const K_ITERATIONS: &str = "20";
    const K_DURATION: &str = "5";

    let mut ctr = 0;
// adding arg parsing

    #[derive(StructOpt, Debug)]
    // #[derive(Debug)]
    #[structopt(rename_all = "kebab-case")]
    struct Opt {
        #[structopt(default_value = K_PORT, short)]
        port: i32,

        #[structopt(default_value = K_HOST, short)]
        host: String,

        #[structopt(default_value = K_ITERATIONS, short)]
        iter: i32,

        #[structopt(default_value = K_DURATION, short)]
        dur: u64,
    }

    let opt = Opt::from_args();
    println!("{:#?}", &opt);

    let connect_string = format!("{}:{}", opt.host, opt.port);

    while ctr < opt.iter {
        thread::sleep(time::Duration::from_secs(opt.dur));
        match TcpStream::connect(&connect_string) {
            Ok(mut stream) => {
                println!("{}: iteration {}: Successfully connected to server on port 3333", Utc::now(), ctr);
                let mut send_string = Vec::new();
                std::write!(&mut send_string, "{}: --------from {} message #>{}<", Utc::now(), K_HOST, ctr).unwrap();
                println!("{}: sending {}", Utc::now(), from_utf8(&send_string).unwrap());
                match stream.write(&send_string) {
                    Ok(n) => {
                        println!("{}: write ok! {} bytes", Utc::now(), n);
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
                            println!("{}: reply is matches! read {} bytes",Utc::now(), n);
                        } else {
                            let text = from_utf8(&data).unwrap();
                            println!("{}: Unexpected reply: {} bytes {}",Utc::now(),n, text);
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
