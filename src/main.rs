fn main() {
    println!("Hello, world!");
    println!("{}", average(2.0, 7.4, 4.6));
    println!("{}", celsius_to_fahrenheit(32.0));
    max_min_mean([2,3,7,13,1,30]);
}

fn average(a: f32, b: f32, c: f32) -> f32 {
    return (a + b + c) / 3.0;
}

fn celsius_to_fahrenheit(temp: f64) -> f64 {
    return temp * 1.8 + 32.0;
}

fn max_min_mean(arr: [i32; 6]) {
    let mut max: i32;
    let mut min: i32;
    let mut mean: f64;

    max = arr[0];
    min = arr[0];
    mean = 0.0;

    for &num in arr.iter() {
        if num > max {
            max = num;
        }
        if num < min {
            min = num
        }
        mean += num as f64;
    }
    mean /= arr.len() as f64;

    println!("{}", min);
    println!("{}", max);
    println!("{}", mean);
}