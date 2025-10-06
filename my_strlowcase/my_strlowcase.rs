fn my_strlowcase(s: &mut str) {
    // s.make_ascii_lowercase();
    let bytes = unsafe { s.as_bytes_mut() };
    bytes.into_iter().for_each(|c| {
        if *c >= b'A' && *c <= b'Z' {
            *c = *c + b'a' - b'A';
        }
    });
}

fn main() {
    let mut s = String::from("Hello World!");
    println!("{}", s);
    my_strlowcase(&mut s);
    println!("{}", s);
}
