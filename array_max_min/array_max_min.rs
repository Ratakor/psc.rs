fn array_max_min(tab: &[i32], max: &mut i32, min: &mut i32) {
    if tab.len() == 0 {
        return;
    }

    *max = tab[0];
    *min = tab[0];

    for i in 1..tab.len() {
        if tab[i] > *max {
            *max = tab[i];
        } else if tab[i] < *min {
            *min = tab[i];
        }
    }
}

fn main() {
    let a = [43, 24, 324, 12, 4, 12, 41, 24, 12, 4];
    let mut max: i32 = 0;
    let mut min: i32 = 0;

    array_max_min(&a, &mut max, &mut min);

    assert_eq!(max, 324);
    assert_eq!(min, 4);
}
