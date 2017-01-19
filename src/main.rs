fn main() {
    println!("{} {} {}", halve(5.0), halve(10.0), halve(15.0));
    let a = halve(16.0);
    let mut b = halve(12.0);
    println!("{} {}", a, b);
    b = 100.0;
    println!("{} {}", a, b);
}

fn halve(x: f32) -> f32 {
    x / 2.0
}
