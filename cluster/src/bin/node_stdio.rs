use std::env;
use std::io::{
    stdin,
    Read,
};

fn main() {
    let mut args = env::args();

    args.next();
    let num = args.next().unwrap();

    println!("process[{}] has spawned", num);

    let mut buf = [0u8; 20];

    let mut s = stdin();

    loop {
        s.read(&mut buf).unwrap();

        println!("process[{}] has awoken", num);
    }
}
