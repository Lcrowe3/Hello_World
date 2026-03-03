fn main() {
    println!("Hello, world!");
    println!("{}", average(2.0, 7.4, 4.6));
    println!("{}", celsius_to_fahrenheit(32.0));
}

fn average(a: f32, b: f32, c: f32) -> f32 {
    return (a + b + c) / 3.0;
}

fn celsius_to_fahrenheit(temp: f64) -> f64 {
    return temp * 1.8 + 32.0;
}