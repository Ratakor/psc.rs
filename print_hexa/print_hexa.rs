fn print_hexa(number: i32) {
    println!("{:#010x}", number)
}

fn main() {
    print_hexa(42);
    print_hexa(1024);
    print_hexa(0);
}
