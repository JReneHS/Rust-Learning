use std::io;

fn main() {
    let mut num = String::new();
    loop {
        io::stdin()
            .read_line(&mut num)
            .expect("Error al leer la linea\n");
        let num: u32 = match num.trim().parse() {
            Ok(rnum) => rnum,
            Err(_) => {
                println!("ingresa un valor numerico...se asigna 3\n");
                num = "3".to_string();
                continue;                
            }
        };
        hanoi(num, 1, 3);
        break;
    }
}

fn hanoi(n: u32, start: u32, end: u32) {
    if n == 1 {
        println!("({}) || {} -> {}",n , start, end);
    } else {
        let other: u32 = 6 - (start + end);
        hanoi(n - 1, start, other);
        println!("({}) || {} -> {}",n , start, end);
        hanoi(n - 1, other, end);
    }
}
