use std::net::TcpListener;
use std::os::unix::io::IntoRawFd;
use std::os::unix::io::FromRawFd;

use libc::pid_t;

extern "C" {
    fn fork() -> pid_t;
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    let tcp_fd = listener.into_raw_fd();

    for i in 0..4 {
        let child_id;
        unsafe {
            child_id = fork();
        }

        if child_id == 0 {
            let new_listener;
            unsafe {
                new_listener = <TcpListener as FromRawFd>::from_raw_fd(tcp_fd);
            }

            for stream in new_listener.incoming() {
                match stream {
                    Ok(_) => {
                        println!("connected: {}", i);
                    },
                    Err(error) => {
                        println!("error: {:?}", error);
                    },
                };
            }

            println!("closed");

            return
        }
    }

    println!("clustering end");
}
