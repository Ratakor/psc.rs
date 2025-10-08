// this is untested see main

fn propagate(map: &[[mut char]], i: i32, j: i32) {
    if (i < 0 || j < 0 || map[i][j] != '.') {
        return;
    }

    map[i][j] = 'o';

    propagate(map, i + 1, j);
    propagate(map, i - 1, j);
    propagate(map, i, j + 1);
    propagate(map, i, j - 1);
}

pub fn lakes(map: &[[mut char]]) -> i32 {
    let mut n = 0;

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == '.' {
                n += 1;
                propagate(map, i, j);
            }
        }
    }

    return n;
}

fn main() {
    // TODO: lakes_example
}
