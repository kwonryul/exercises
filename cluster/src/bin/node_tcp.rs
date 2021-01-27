use std::env;
use std::str::FromStr;
use std::os::unix::io::FromRawFd;
use std::net::TcpListener;

fn main() {
    let mut args = env::args();

    args.next();
    let fd = args.next().unwrap();
    let fd = <i32 as FromStr>::from_str(&fd).unwrap();
    let num = args.next().unwrap();

    let listener;
    unsafe {
        listener = <TcpListener as FromRawFd>::from_raw_fd(fd);
    }

    for stream in listener.incoming() {
        match stream {
            Ok(_) => {
                println!("connected: {}", num);
            },
            Err(error) => {
                println!("{} : error: {:?}", num, error);
            }
        };
    }

    println!("closed");
}
