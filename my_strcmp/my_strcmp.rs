// this is streql not strcmp but idc tbh

pub fn my_strcmp(s1: &str, s2: &str) -> bool {
    return s1 == s2;
}

fn main() {
    assert_eq!(my_strcmp("abc", "abc"), true);
    assert_eq!(my_strcmp("abc", &String::from("abc")), true);
    assert_eq!(my_strcmp("abc", "bac"), false);
    assert_eq!(my_strcmp("abc", "abcd"), false);
}
