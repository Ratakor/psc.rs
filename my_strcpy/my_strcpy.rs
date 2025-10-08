// This looks very wrong

pub fn my_strcpy(dest: &mut String, source: &str) {
    *dest = String::from(source);
}

fn main() {
    let mut dst: String = Default::default();
    let src = "Hello World!";

    my_strcpy(&mut dst, src);

    println!("{}", dst);
}
