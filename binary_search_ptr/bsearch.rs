// I hate ptr in rust so I did it with indexes

pub fn binary_search(arr: &[i32], elt: i32) -> usize {
    let mut begin = 0;
    let mut end = arr.len() - 1;

    while begin <= end {
        let mid = (end + begin) / 2;

        if arr[mid] == elt {
            return mid;
        }

        if arr[mid] < elt {
            begin = mid + 1;
        } else {
            if mid == 0 {
                break;
            }
            end = mid - 1;
        }
    }

    begin
}

fn main() {
    let a = [0, 1, 4, 5, 9, 10, 18, 22, 42, 51, 69];
    assert_eq!(binary_search(&a, 5), 3);
    assert_eq!(binary_search(&a, 0), 0);
    assert_eq!(binary_search(&a, -1), 0);
    assert_eq!(binary_search(&a, 99), 11);
    assert_eq!(binary_search(&a, 68), 10);
    assert_eq!(binary_search(&a, 69), 10);
}
