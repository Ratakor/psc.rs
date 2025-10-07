pub fn number_digits_rec(n: u32) -> u32 {
    if n < 10 {
        1
    } else {
        1 + number_digits_rec(n / 10)
    }
}

fn main() {
    println!("{}", number_digits_rec(123456));
}
