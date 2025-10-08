pub fn greatest_divisor(n: u32) -> u32 {
    let mut max = 1;
    for i in 2..n - 1 {
        if n % i == 0 {
            max = i;
        }
    }
    return max;
}

fn main() {
    assert_eq!(greatest_divisor(18), 9);
    assert_eq!(greatest_divisor(25), 5);
    assert_eq!(greatest_divisor(97), 1);
}
