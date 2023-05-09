use std::collections::VecDeque;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(short = "p", long = "port", default_value = "8080")]
    port: u16,
}

fn main() {
    let args = Cli::from_args();
    if let Ok(listener) = TcpListener::bind(format!("127.0.0.1:{}", args.port)) {
        let queue = Arc::new(Mutex::new(VecDeque::new()));

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let queue_ref = queue.clone();
                    thread::spawn(move || {
                        handle_client(stream, queue_ref);
                    });
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
    } else {
        println!("無法binding ")
    }
}

fn handle_client(mut stream: TcpStream, queue: Arc<Mutex<VecDeque<String>>>) {
    let mut buffer = [0; 1024];
    let mut data = String::new();

    loop {
        match stream.read(&mut buffer) {
            Ok(size) => {
                if size == 0 {
                    break;
                }
                let received = String::from_utf8_lossy(&buffer[..size]);
                data.push_str(&received);
                println!("{}", received);
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
        }
    }

    let mut queue = queue.lock().unwrap();
    queue.push_back(data);
    stream.write(b"Data received successfully!\n").unwrap();
}