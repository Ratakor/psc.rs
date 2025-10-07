// normally return a &mut i32 but idk how to make it do that
// and I cba fighting the compiler
fn add_int_ptr(a: &mut i32, b: &i32) {
    // a and b can't be null
    // I won't use Option type for this
    *a += *b;
    // return a;
}

fn main() {
    let mut a = 15;
    let b = 6;
    add_int_ptr(&mut a, &b);
    println!("a = {}", a);
    println!("b = {}", b);
    // println!("c = {}", *c);
}
