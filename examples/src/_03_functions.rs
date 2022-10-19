fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn diff(a: i32, b: i32) -> i32 {
    return a - b;
}
#[allow(dead_code)]
pub fn example() {
    let a = 10;
    let b = 20;
    println!("{} + {} = {}", a, b, sum(a, b));
    println!("{} - {} = {}", a, b, diff(a, b));
}
