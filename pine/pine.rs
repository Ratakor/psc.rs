use std::process::exit;

pub fn pine(n: usize) -> i32 {
    if n < 3 {
        return 1;
    }

    for i in 0..n {
        println!(
            "{}{}*{}",
            " ".repeat(n - i - 1),
            "*".repeat(i),
            "*".repeat(i)
        );
    }

    for _ in 0..n / 2 {
        println!("{}*", " ".repeat(n - 1));
    }

    return 0;
}

fn main() {
    exit(pine(4));
}
