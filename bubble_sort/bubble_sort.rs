fn bubble_sort(array: &mut [i32]) {
    if array.len() == 0 {
        return;
    }

    for i in (0..array.len()).rev() {
        for j in 0..i {
            if array[j + 1] < array[j] {
                let tmp = array[j + 1];
                array[j + 1] = array[j];
                array[j] = tmp;
            }
        }
    }
}

fn main() {
    let mut array = [6, 1, 8, 5, 4];

    bubble_sort(&mut array);

    assert_eq!(array[0], 1);
    assert_eq!(array[1], 4);
    assert_eq!(array[2], 5);
    assert_eq!(array[3], 6);
    assert_eq!(array[4], 8);
}
