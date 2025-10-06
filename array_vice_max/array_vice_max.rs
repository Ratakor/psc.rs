fn array_vice_max(vec: &[i32]) -> i32 {
    let mut max: i32;
    let mut vice: i32;

    if vec[0] > vec[1] {
        max = vec[0];
        vice = vec[1];
    } else {
        max = vec[1];
        vice = vec[0];
    }

    for i in 2..vec.len() {
        if vec[i] > max {
            vice = max;
            max = vec[i];
        } else if vec[i] > vice {
            vice = vec[i];
        }
    }

    return vice;
}

fn main() {
    let vec = [43, 24, 324, 12, 4, 12, 41, 24, 12, 4];
    let vice = array_vice_max(&vec);
    assert_eq!(vice, 43);
    println!("{}", vice);
}
