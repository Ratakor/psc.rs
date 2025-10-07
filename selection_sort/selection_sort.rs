pub fn array_min(arr: &[i32], start: usize) -> usize {
    let mut min = start;
    for i in start + 1..arr.len() {
        if arr[i] < arr[min] {
            min = i;
        }
    }
    return min;
}

pub fn selection_sort(arr: &mut [i32]) {
    for i in 0..arr.len() {
        let min = array_min(arr, i);
        if min != i {
            let tmp = arr[i];
            arr[i] = arr[min];
            arr[min] = tmp;
        }
    }
}

fn main() {
    let mut arr = [
        5, 8, 90, 3, 7, 64, 10224, 88, 39, 78, 20, 6, 9, 79, 30, 45, 908, 201, 73, 460, 1330, 37,
        32, 13, 709, 310, 1998, 2000, 2020, 2021, 2022, 5600, 10000, 4560, 4800,
    ];
    let expected = [
        3, 3, 3, 3, 11, 11, 11, 11, 11, 11, 11, 11, 12, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23,
        25, 25, 26, 27, 28, 29, 30, 33, 33, 33, 34,
    ];

    for i in 0..arr.len() {
        assert_eq!(array_min(&arr, i), expected[i]);
    }

    selection_sort(&mut arr);

    // check if arr is sorted
    arr.windows(2).all(|w| w[0] <= w[1]);
}
