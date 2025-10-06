use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("Hello World!");
    } else {
        for i in 1..args.len() {
            println!("Hello {}!", args[i]);
        }
    }
}
