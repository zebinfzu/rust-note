// 1
fn convert_temperatures_fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

// 2
fn fib(n: u64) -> u64 {
    match n {
        a @ 0..=2 => a % 1,
        n => fib(n - 1) + fib(n - 2)
    }
} 

fn main() {
    let f = 200.0;
    println!("{}", convert_temperatures_fahrenheit_to_celsius(f));
    println!("{}", fib(5));
}
