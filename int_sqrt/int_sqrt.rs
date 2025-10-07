fn int_sqrt(n: i32) -> i32 {
    if n < 0 {
        return -1;
    }

    let mut x = n;
    let mut y = (x + 1) / 2;
    while y < x {
        x = y;
        y = (x + n / x) / 2;
    }

    x
}

fn main() {
    println!("{}", int_sqrt(17));
}
