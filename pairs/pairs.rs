#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Pair(i32, i32);

// TODO: impl Add
// it's given by copy in subject too
pub fn three_pairs_sum(p1: Pair, p2: Pair, p3: Pair) -> Pair {
    Pair(p1.0 + p2.0 + p3.0, p1.1 + p2.1 + p3.1)
}

// TODO: impl Sum
pub fn pairs_sum(pairs: &[Pair]) -> Pair {
    let mut sum = Pair(0, 0);
    for p in pairs {
        sum.0 += p.0;
        sum.1 += p.1;
    }
    sum
}

fn main() {
    let p1 = Pair(1, 2);
    let p2 = Pair(1, 2);
    let p3 = Pair(1, 2);

    let pairs = [p1, p2, p3];

    assert_eq!(three_pairs_sum(p1, p2, p3), pairs_sum(&pairs));
}
