fn main() {
    let num = 3;
    hanoi(num, 1, 3);
}

fn hanoi(n: i32, start: i32, end: i32) {
    if n == 1 {
        println!("{} -> {}", start, end);
    } else {
        let other: i32 = 6 - (start + end);
        hanoi(n - 1, start, other);
        println!("{} -> {}", start, end);
        hanoi(n - 1, other, end);
    }
}
