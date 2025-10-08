pub fn create_array(size: usize) -> Vec<i32> {
    Vec::<i32>::with_capacity(size)
}

pub fn free_array(arr: &Vec<i32>) {
    // I guess
    _ = arr;
}

pub fn read_and_inc(v: &mut i32) {
    println!("{}", *v);
    *v += 1;
}

pub fn my_strdup(s: &str) -> String {
    // s.clone() would only return a new ref not a new allocated instance
    String::from(s)
}

pub fn my_strndup(s: &str, n: usize) -> String {
    let size = std::cmp::min(s.len(), n);
    s[0..size].to_string()
}

fn main() {
    let mut arr = create_array(8);

    arr.push(3);
    // arr[0] = 3;
    read_and_inc(&mut arr[0]);
    read_and_inc(&mut arr[0]);
    // read_and_inc(&mut arr[1]); // will crash

    let s = "sauce";
    let dup = my_strdup(s);
    let ndup = my_strndup(s, 3);

    assert_eq!(dup, "sauce");
    assert_eq!(ndup, "sau");

    free_array(&arr);
}
