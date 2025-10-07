fn fibo_iter(n: u64) -> u64 {
    let mut u0 = 0;
    let mut u1 = 1;
    for _ in 1..=n {
        let tmp = u0 + u1;
        u0 = u1;
        u1 = tmp;
    }
    u0
}

fn main() {
    println!("{}", fibo_iter(15));
}
