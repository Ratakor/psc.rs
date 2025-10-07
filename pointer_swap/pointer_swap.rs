fn pointer_swap<'a>(a: &mut &'a i32, b: &mut &'a i32) {
    let tmp = *a;
    *a = *b;
    *b = tmp;
}

fn main() {
    let a = 42;
    let b = 69;

    let mut pa = &a;
    let mut pb = &b;

    println!("{} {}", *pa, *pb);

    pointer_swap(&mut pa, &mut pb);

    println!("{} {}", *pa, *pb);
}
