fn hanoi_rec(n: u32, a: char, b: char, c: char) {
    if n >= 1 {
        hanoi_rec(n - 1, a, c, b);
        println!("{}->{}", a, c);
        hanoi_rec(n - 1, b, a, c);
    }
}

pub fn hanoi(n: u32) {
    hanoi_rec(n, '1', '2', '3');
}

fn main() {
    hanoi(3);
}
