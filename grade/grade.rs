fn grade(g: char) {
    match g {
        'A' => println!("Excellent"),
        'B' => println!("Good"),
        'C' => println!("Not so bad"),
        'D' => println!("Could be worse"),
        'E' => println!("Maybe next time"),
        'F' => println!("No comment"),
        _ => println!("Call a wild ACU"),
    }
}

fn main() {
    grade('A');
    grade('B');
    grade('C');
    grade('D');
    grade('E');
    grade('F');
    grade('G');
}
