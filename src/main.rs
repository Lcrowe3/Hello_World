fn main() {
    println!("Hello, world!");
    println!("{}", average(2.0, 7.4, 4.6));
}

fn average(a: f32, b: f32, c: f32) -> f32 {
    return (a + b + c) / 3.0;
}