//! Simple Rust TCP Client
//! Dwight J. Browne
//! Will either accept arguments from the command line or ENV vars
//! -p  port #  -h hostname  -i #of messages to be sent  -d duration of delay between messages
//! ENV VARS are
//! CLIENT_PORT,  CLIENT_HOST, CLIENT_DURATION, CLIENT_ITERATIONS
//!


use std::{thread, time};
use std::env;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;

use chrono::Utc;
use structopt::StructOpt;

fn main() {
    const K_PORT: &str = "3333";
    const K_HOST: &str = "localhost";
    const K_BUFSIZ: usize = 1024;
    const K_ITERATIONS: &str = "2000";
    const K_DURATION: &str = "5";

    let mut ctr = 0;

    // check for command line args
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

    let _port: i32;
    let _host: &str;
    let _iterations: i32;
    let _duration: u64;

    // check for environment variables
    let env_port = env::var("CLIENT_PORT").unwrap_or(K_PORT.to_string());
    let env_host = env::var("CLIENT_HOST").unwrap_or(K_HOST.to_string());
    let env_dur = env::var("CLIENT_DURATION").unwrap_or(K_DURATION.to_string());
    let env_iteration = env::var("CLIENT_ITERATIONS").unwrap_or(K_ITERATIONS.to_string());

    // see if we have any input args
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        println!("Have input args: Overriding default vars with the following: ENV VARS IGNORED!");
        println!("{:#?}", &opt);
        _port = opt.port;
        _host = &opt.host;
        _iterations = opt.iter;
        _duration = opt.dur;
    } else {
        println!("Using environment vars: with the following values");
        println!("port:           {}", env_port);
        println!("Host:           {}", env_host);
        println!("delay duration: {}", env_dur);
        println!("# of Iterations: {}", env_iteration);
        _port = env_port.parse::<i32>().unwrap();
        _host = &*env_host;
        _iterations = env_iteration.parse::<i32>().unwrap();
        _duration = env_dur.parse::<u64>().unwrap();
    }


    let connect_string = format!("{}:{}", &_host, &_port);

    while ctr < _iterations {
        match TcpStream::connect(&connect_string) {
            Ok(mut stream) => {
                println!("{}: iteration {}: Successfully connected to server on port 3333", Utc::now(), ctr);
                let mut send_string = Vec::new();
                std::write!(&mut send_string, "{}: --------from {} message #>{}<", Utc::now(), K_HOST, ctr).unwrap();

                println!("{}: sending {}", Utc::now(), from_utf8(&send_string).unwrap());
                match stream.write(&send_string) {
                    Ok(n) => {
                        println!("{}: write ok! {} bytes", Utc::now(), n);
                    }
                    Err(e) => {
                        println!("{}: Canna write the data Captn'! {}", Utc::now(), e);
                    }
                }

                match stream.flush() {
                    Ok(_) => {}
                    Err(e) => {
                        println!("{}: Canna flush it {}", Utc::now(), e);
                    }
                }
                println!("{}: Sent {} Waiting for reply....", Utc::now(), from_utf8(&send_string).unwrap());
                let mut data = [0 as u8; K_BUFSIZ];
                match stream.read(&mut data) {
                    Ok(n) => {
                        if &data[..n] == &send_string[..] {
                            println!("{}: reply matches! read {} bytes", Utc::now(), n);
                        } else {
                            let text = from_utf8(&data).unwrap();
                            println!("{}: Unexpected reply: {} bytes {}", Utc::now(), n, text);
                        }
                    }
                    Err(e) => {
                        println!("{}: failed to receive data {}", Utc::now(), e);
                    }
                }
            }
            Err(e) => {
                println!("{}: Failed to connect: {}!", Utc::now(), e);
            }
        }
        ctr += 1;
        thread::sleep(time::Duration::from_secs(_duration));
        println!("");
    }
    println!("{}: Terminated.", Utc::now());
}
