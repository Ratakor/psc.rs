pub fn str_revert(s: &str) -> String {
    s.chars().rev().collect::<String>()
}

fn main() {
    assert_eq!(str_revert("fou"), "uof");
}
