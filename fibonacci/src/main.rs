fn main() {
    let val = 5;
    println!("Fibonacci  of {} is {}", val, fibonacci(val));
}
fn fibonacci(num: i64) -> i64 {
    if num <= 1 {
        1
    } else {
        num + fibonacci(num - 1)
    }
}
