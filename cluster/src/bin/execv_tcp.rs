use std::str::FromStr;
use std::os::unix::io::FromRawFd;

use std::net::TcpListener;
use std::os::unix::io::IntoRawFd;
use std::ptr::null;

use libc::{
    pid_t,
    c_char,
    c_int,
    F_GETFD,
    F_SETFD,
    FD_CLOEXEC,
};

extern "C" {
    fn fork() -> pid_t;
    fn execv(prog: *const c_char, argv: *const *const c_char) -> c_int;
    fn fcntl(fd: c_int, cmd: c_int, ...) -> c_int;
}

fn main() {
    println!("started");
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    let tcp_fd = listener.into_raw_fd();
    let mut env_fd = tcp_fd.to_string().into_bytes();
    env_fd.push(0u8);
    let env_fd = env_fd.as_slice() as *const [u8] as *const i8;

    unsafe {
        let stat = fcntl(tcp_fd, F_GETFD, 0);
        if stat & FD_CLOEXEC != 0 {
            println!("will close");
            fcntl(tcp_fd, F_SETFD, 0x0);
            println!("modified");
        } else {
            println!("will not close");
        };
    }

    for i in 0..4 {
        let mut num = i.to_string().into_bytes();
        num.push(0u8);
        let num = num.as_slice() as *const [u8] as *const i8;

        let child_id;
        unsafe {
            child_id = fork();
        }

        if child_id == 0 {
            let bin_path = b"target/debug/node_tcp\x00" as *const _ as *const i8;

            let null_ptr = null::<i8>();
            let argv = [bin_path, env_fd, num, null_ptr];
            let argv = &argv as *const *const c_char;

            unsafe {
                execv(bin_path, argv);
            }

            println!("failed");
            return
        }
    }

    println!("clustering end");
}
