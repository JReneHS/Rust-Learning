fn main() {
    let oe1 = odd(5);
    let oe2 = odd(12);

    println!("5 es par?: {}", oe1);
    println!("6 es par?: {}\n", oe2);

    let oe1: (f32, f64) = tuple_demo((4, 5, 6));
    println!("Tuple Demo >> {:?}", oe1);
    println!("Factorial de 11 >> {}\n", fact(11));

    let mut counter = 0;
    let res_loop = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("ResLoop >> {}\n", res_loop);

    let mut counter = 5;
    while counter != 0 {
        println!("{}", counter);
        counter -= 1;
    }
    println!();
    let a = [10, 20, 75, 2, 5, 7];

    // Es un For auto
    for element in a.iter() {
        println!("The element is >> {}", element);
    }

    println!();
    for number in (1..5).rev() {
        println!("{}", number);
    }

    println!();
    for nums in 3..9 {
        println!("{}", nums);
    }
}

fn fact(n: i32) -> i32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fact(n - 1) * n,
    }
}

fn odd(n: i32) -> bool {
    (n & 1) == 0
}

fn tuple_demo(t: (u8, u16, u32)) -> (f32, f64) {
    let x: f32 = t.0 as f32 + t.1 as f32;
    let y: f64 = t.2 as f64;
    (x, y)
}
