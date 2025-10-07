fn digit(mut n: i32, k: i32) -> u32 {
    if n <= 0 || k <= 0 {
        return 0;
    }

    for _ in 1..k {
        n /= 10;
    }

    (n % 10) as u32
}

fn main() {
    assert_eq!(5, digit(123456, 2));
    assert_eq!(3, digit(123456, 4));
    assert_eq!(0, digit(123456, 7));
    assert_eq!(0, digit(-123456, 4));
    assert_eq!(0, digit(123456, -4));
}
