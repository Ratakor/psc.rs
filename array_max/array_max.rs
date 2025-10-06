fn array_max(array: &[i32]) -> i32 {
    let mut max = i32::MIN;
    for i in 0..array.len() {
        if array[i] > max {
            max = array[i];
        }
    }
    max
}

fn main() {
    assert_eq!(array_max(&[1, 2, 3, 4, 5]), 5);
    assert_eq!(array_max(&[10, 9, 8, 7, 6]), 10);
    assert_eq!(array_max(&[-4, 3, 9, 42, -13]), 42);
    assert_eq!(array_max(&[-3, -2, -1, -4, -5]), -1);
}
