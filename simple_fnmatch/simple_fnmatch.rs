// TODO: use &str and remove b['"]
pub fn simple_fnmatch(pattern: &[u8], string: &[u8]) -> bool {
    let mut i = 0;
    let mut j = 0;
    while i < pattern.len() {
        match pattern[i] {
            b'?' => {
                if j == string.len() {
                    return false;
                }
            }
            b'*' => {
                while i < pattern.len() && pattern[i] == b'*' {
                    i += 1;
                }
                if i == pattern.len() {
                    return true;
                }
                while j < string.len() {
                    if simple_fnmatch(&pattern[i..], &string[j..]) {
                        return true;
                    }
                    j += 1;
                }
                return false;
            }
            b'\\' => {
                i += 1;
                if i == pattern.len() || j == string.len() || pattern[i] != string[j] {
                    return false;
                }
            }
            _ => {
                if j == string.len() || pattern[i] != string[j] {
                    return false;
                }
            }
        }

        i += 1;
        j += 1;
    }

    j == string.len()
}

fn main() {
    assert_eq!(false, simple_fnmatch(b"pattern", b"string"));
    assert_eq!(true, simple_fnmatch(b"/home", b"/home"));
    assert_eq!(true, simple_fnmatch(b"*.txt", b"text.txt"));
    assert_eq!(false, simple_fnmatch(b"*.txt", b"text.t2xt"));
    assert_eq!(false, simple_fnmatch(b"*.txt", b"text."));
    assert_eq!(false, simple_fnmatch(b"*.txt", b"text"));
    assert_eq!(false, simple_fnmatch(b"*.txt", b""));
    assert_eq!(true, simple_fnmatch(b"*", b"asfkjdhfajsh"));
    assert_eq!(true, simple_fnmatch(b"p?ttern", b"p2ttern"));
    assert_eq!(true, simple_fnmatch(b"p\\?ttern", b"p?ttern"));
    assert_eq!(false, simple_fnmatch(b"p\\?ttern\\", b"p?ttern\\"));
    assert_eq!(true, simple_fnmatch(b"", b""));
    assert_eq!(true, simple_fnmatch(b"*", b""));
    assert_eq!(false, simple_fnmatch(b"*.x.*", b"test"));
    assert_eq!(true, simple_fnmatch(b"*.x.*", b"sauce.x."));
    assert_eq!(false, simple_fnmatch(b"*.x.*", b"sauce.x"));
    assert_eq!(true, simple_fnmatch(b"*.x.*", b"sauce.x.s"));
    assert_eq!(false, simple_fnmatch(b"*.*dahak", b"sauce.x.s"));
    assert_eq!(true, simple_fnmatch(b"*.*dahak", b".dahak"));
    assert_eq!(true, simple_fnmatch(b"*.*dahak", b".xmdrdahak"));
    assert_eq!(true, simple_fnmatch(b"*.*dahak", b"fsfd...xmdrdahak"));
    assert_eq!(true, simple_fnmatch(b"*.*dahak", b"fsfd.xmdrdahak"));
    assert_eq!(true, simple_fnmatch(b"*.*dah*ak", b"fsfd.xmdrdahak"));
    assert_eq!(true, simple_fnmatch(b"*.*dahak", b"daas.dahak"));
    assert_eq!(false, simple_fnmatch(b"jsp*", b"ouaip"));
    assert_eq!(true, simple_fnmatch(b"jsp*", b"jsp"));
    assert_eq!(false, simple_fnmatch(b"jsp*", b"js"));
    assert_eq!(true, simple_fnmatch(b"jsp*", b"jspouai"));
    assert_eq!(false, simple_fnmatch(b"*jsp", b"ouaip"));
    assert_eq!(true, simple_fnmatch(b"*jsp", b"jsp"));
    assert_eq!(false, simple_fnmatch(b"*jsp", b"js"));
    assert_eq!(false, simple_fnmatch(b"*jsp", b"jspouai"));
    assert_eq!(false, simple_fnmatch(b"?", b"jspouai"));
    assert_eq!(true, simple_fnmatch(b"?", b"j"));
    assert_eq!(true, simple_fnmatch(b"?", b"?"));
    assert_eq!(true, simple_fnmatch(b"\\?", b"?"));
    assert_eq!(false, simple_fnmatch(b"?", b""));
    assert_eq!(false, simple_fnmatch(b"j*?p", b"jspouai"));
    assert_eq!(false, simple_fnmatch(b"??jsp", b"jspouai"));
    assert_eq!(false, simple_fnmatch(b"??jsp", b"ajspouai"));
    assert_eq!(false, simple_fnmatch(b"??jsp", b"aajspouai"));
}
