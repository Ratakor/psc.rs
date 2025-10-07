pub fn top_of_the_hill(tab: &[i32]) -> i32 {
    if tab.len() == 0 {
        return -1;
    }

    let mut max = 0;
    for i in 1..tab.len() {
        if tab[i] < 0 {
            return -1;
        }

        if tab[i] > tab[max] {
            max = i;
        }
    }

    for i in 1..max {
        if tab[i] < tab[i - 1] {
            return -1;
        }
    }

    for i in (max + 1..tab.len()).rev() {
        if tab[i] > tab[i - 1] {
            return -1;
        }
    }

    return max as i32;
}

fn main() {
    let tab1 = [1, 2, 3, 4, 6, 6, 4, 2, 0, 0];
    assert_eq!(4, top_of_the_hill(&tab1));

    let tab2 = [1, 2, 3, 4, 5, 6, 6, 6, 6, 6];
    assert_eq!(5, top_of_the_hill(&tab2));

    let tab3 = [1, 2, 3, 4, 6, 6, 4, 5, 0, 0];
    assert_eq!(-1, top_of_the_hill(&tab3));
}
