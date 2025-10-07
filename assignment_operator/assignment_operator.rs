fn plus_equal(a: &mut i32, b: &i32) {
    *a += *b;
}

fn minus_equal(a: &mut i32, b: &i32) {
    *a -= *b;
}

fn mult_equal(a: &mut i32, b: &i32) {
    *a *= *b;
}

fn div_equal(a: &mut i32, b: &i32) -> i32 {
    let rmd = *a % *b;
    *a /= *b;
    rmd
}

fn main() {
    let mut a: i32;
    let mut b: i32;

    a = 5;
    b = 3;
    plus_equal(&mut a, &b);
    println!("{}", a);

    a = 5;
    b = 3;
    minus_equal(&mut a, &b);
    println!("{}", a);

    a = 5;
    b = 3;
    mult_equal(&mut a, &b);
    println!("{}", a);

    a = 5;
    b = 3;
    let rmd = div_equal(&mut a, &b);
    println!("{} {}", a, rmd);
}
