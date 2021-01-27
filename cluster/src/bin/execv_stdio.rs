use std::io::{
    stdin,
    Read,
};
use std::thread;
use std::time::Duration;

use std::ptr::null;

use libc::{
    pid_t,
    c_char,
    c_int,
    size_t,
    ssize_t,
    c_void,
};

extern "C" {
    fn close(fd: c_int) -> c_int;
    fn fork() -> pid_t;
    fn execv(prog: *const c_char, argv: *const *const c_char) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
}

fn main() {
    let mut first = true;

    for i in 0..4 {
        let mut num = i.to_string().into_bytes();
        num.push(0u8);
        let num = num.as_slice() as *const [u8] as *const i8;

        let child_id;
        unsafe {
            child_id = fork();
        }

        if first == true {
            first = false;
            if child_id == 0 {
                let mut s = stdin();
                let mut buf = [0u8; 20];
                while let Ok(_) = s.read(&mut buf) {
                    println!("special process has awoken");
                }
                println!("{:?}", s.read(&mut buf));

                let b = &mut buf as *mut _ as *mut c_void;
                loop {
                    unsafe {
                        println!("{}", read(0, b, 1));
                    }
                }
            }
        }
        if child_id == 0 {
            let bin_path = b"target/debug/node_stdio\x00" as *const _ as *const i8;

            let null_ptr = null::<i8>();

            let argv = [bin_path, num, null_ptr];
            let argv = &argv as *const *const c_char;

            unsafe {
                execv(bin_path, argv);
            }

            println!("failed");
            return
        }
    }

    println!("clustering end");

    let mut buf = [0u8; 20];
    let mut s = stdin();

    for j in 0..5 {
        s.read(&mut buf).unwrap();
        println!("main process has awoken");
    }


    unsafe {
        close(0);
    }

    let len = s.read(&mut buf).unwrap();
    println!("len is {}, as expected : closed", len);

    println!("you have 5 secs");
    thread::sleep(Duration::new(5, 0));
    println!("ended");
}
