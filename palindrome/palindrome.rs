pub fn palindrome(s: &str) -> bool {
    if s.len() == 0 {
        return true;
    }

    let mut left = 0;
    let mut right = s.len() - 1;

    loop {
        // rustc want me to initialize these values
        let mut c1 = '\0';
        let mut c2 = '\0';
        while left != right {
            c1 = s.chars().nth(left).unwrap();
            if (c1 >= 'a' && c1 <= 'z') || (c1 >= 'A' && c1 <= 'Z') || (c1 >= '0' && c1 <= '9') {
                break;
            }
            left += 1;
        }
        while right != left {
            c2 = s.chars().nth(right).unwrap();
            if (c2 >= 'a' && c2 <= 'z') || (c2 >= 'A' && c2 <= 'Z') || (c2 >= '0' && c2 <= '9') {
                break;
            }
            right -= 1;
        }
        if left >= right {
            break;
        }

        if c1 != c2 {
            return false;
        }

        left += 1;
        right -= 1;
    }

    true
}

fn main() {
    assert_eq!(false, palindrome("kayal"));
    assert_eq!(true, palindrome("kayak"));
    assert_eq!(true, palindrome("karine alla en irak"));
    assert_eq!(true, palindrome("k a y a k  "));
    assert_eq!(true, palindrome("Tu l'aS troP ete, Port-SaluT."));
    assert_eq!(true, palindrome("tu l'as trop ete, port-salut."));
    assert_eq!(true, palindrome(""));
    assert_eq!(false, palindrome("Tu l'aS troP ete, port-SaluT."));
    assert_eq!(false, palindrome("r0xr"));
    assert_eq!(false, palindrome("kayAK"));
}
