fn q_rsqrt(num: f32) -> f32 {
    let i = num.to_bits();
    let i = 0x5f3759df - (i >> 1);
    let y = f32::from_bits(i);

    y * (1.5 - 0.5 * num * y * y)
}

fn print_both(v: f32) {
    println!("quake: {}", q_rsqrt(v));
    println!("real:  {}", 1.0 / v.sqrt());
    println!();
}

fn main() {
    println!("Inverse square Root");
    print_both(4.0);
    print_both(10.0);
    print_both(3.1415);
}
