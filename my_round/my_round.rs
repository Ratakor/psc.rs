fn my_round(n: f32) -> i32 {
    let dec = n as i32;
    let frac = n - dec as f32;

    if n > 0.0 {
        if frac < 0.5 {
            dec
        } else {
            dec + 1
        }
    } else {
        if frac <= -0.5 {
            dec - 1
        } else {
            dec
        }
    }
}

fn main() {
    assert_eq!(my_round(42.0), 42);
    assert_eq!(my_round(-42.0), -42);
    assert_eq!(my_round(42.1), 42);
    assert_eq!(my_round(-42.1), -42);
    assert_eq!(my_round(42.5), 43);
    assert_eq!(my_round(-42.5), -43);
    assert_eq!(my_round(42.49), 42);
    assert_eq!(my_round(-42.49), -42);
    assert_eq!(my_round(42.51), 43);
    assert_eq!(my_round(-42.51), -43);
    assert_eq!(my_round(42.9), 43);
    assert_eq!(my_round(-42.9), -43);
}
