fn element_count(begin: *const i32, end: *const i32) -> usize {
    return unsafe { end.offset_from(begin) as usize };
}

fn main() {
    let arr = [1, 2, 3, 4];
    let begin = arr.as_ptr();
    let end = arr.as_ptr().wrapping_add(4);
    println!("{}", element_count(begin, end));
}
