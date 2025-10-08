pub fn int_palindrome(number: i32) -> bool {
    if number < 0 {
        return false;
    }

    let mut n = number;
    let mut reversed = 0;
    while n != 0 {
        reversed *= 10;
        reversed += n % 10;
        n /= 10;
    }

    reversed == number
}

fn main() {
    assert_eq!(false, int_palindrome(42));
    assert_eq!(true, int_palindrome(242));
    assert_eq!(true, int_palindrome(242242));
    assert_eq!(false, int_palindrome(242442));
    assert_eq!(false, int_palindrome(-101));
}
