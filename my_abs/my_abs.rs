// can't do in 1 line because rustfmt
fn my_abs(n: i32) -> i32 {
    if n >= 0 {
        n
    } else {
        -n
    }
}

fn main() {
    println!("{}", my_abs(42));
    println!("{}", my_abs(-42));
}
