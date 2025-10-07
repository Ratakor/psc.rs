use std::env;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        exit(1);
    }

    let s = &args[1];
    let n = args[2].bytes().nth(0).unwrap_or_else(|| exit(1)) - b'0';

    for _ in 0..n {
        println!("{}", s);
    }
}
