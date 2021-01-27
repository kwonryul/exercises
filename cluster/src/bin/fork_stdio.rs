use std::io::{
    stdin,
    Read,
};
use std::thread;
use std::time::Duration;
use libc::pid_t;

extern "C" {
    fn fork() -> pid_t;
}

fn main() {
    for i in 0..4 {
        let child_id;
        unsafe {
            child_id = fork();
        }

        if child_id == 0 {
            let mut buf = [0u8; 20];
            let mut s = stdin();

            loop {
                s.read(&mut buf).unwrap();
                println!("awake! {}", i);
            }
        }
    }

    println!("clustering end");

    for _i in 0..3 {
        thread::sleep(Duration::new(3, 0));
        println!("gonna die");
    }
}
