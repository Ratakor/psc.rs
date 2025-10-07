pub fn my_pow(mut a: i32, mut b: i64) -> i32 {
    let mut res = 1;
    while b != 0 {
        if b % 2 == 0 {
            a *= a;
            b /= 2;
        } else {
            res *= a;
            b -= 1;
        }
    }
    res
}

fn main() {
    assert_eq!(my_pow(1, 2), 1);
    assert_eq!(my_pow(2, 4), 16);
    assert_eq!(my_pow(3, 2), 9);
}
