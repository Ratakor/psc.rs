pub fn factorial(n: u32) -> u64 {
    if n == 0 {
        1
    } else {
        n as u64 * factorial(n - 1)
    }
}

fn main() {
    println!("{}", factorial(5));
}
