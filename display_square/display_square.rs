fn display_square(width: i32) {
    if width <= 0 {
        return;
    }

    let mut w = width as usize;

    if width == 1 {
        println!("*");
        return;
    }

    if w % 2 == 0 {
        w += 1;
    }

    // println!("{:*<1$}", "", w); // no allocation solution
    println!("{}", "*".repeat(w));

    let rows = (w + 1) / 2 - 2;
    for _ in 0..rows {
        println!("*{}*", " ".repeat(w - 2));
    }

    println!("{}", "*".repeat(w));
}

fn main() {
    display_square(6);
}
