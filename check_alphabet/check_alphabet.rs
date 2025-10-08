pub fn check_alphabet(s: &str, alphabet: &str) -> bool {
    for a in alphabet.bytes() {
        let mut found = false;
        for c in s.bytes() {
            if c == a {
                found = true;
                break;
            }
        }
        if !found {
            return false;
        }
    }

    true
}

fn main() {
    assert_eq!(check_alphabet("toto", ""), true);
    assert_eq!(check_alphabet("", "t"), false);
    assert_eq!(check_alphabet("toto asticot", "otaisc k"), false);
    assert_eq!(check_alphabet("toto asticot", "ot"), true);
}
