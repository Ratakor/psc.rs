fn get_int_value(n: &i32) -> i32 {
    return *n;
}

fn main() {
    let n = 42;
    println!("{}", get_int_value(&n));
}
