fn my_strlen(s: &str) -> usize {
    return s.len();
}

fn main() {
    let len = my_strlen("string");
    assert_eq!(len, 6);
    println!("{}", len);
}
