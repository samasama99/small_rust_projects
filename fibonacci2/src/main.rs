pub fn get_fibonacci(n: u32) -> u32 {
    match n {
        0 | 1 => 1,
        n => get_fibonacci(n - 1) + get_fibonacci(n - 2),
    }
}

pub fn fibonacci(n: u32) -> u32 {
    get_fibonacci(n - 1)
}

fn main() {
    println!(
        "The element in the position {} in fibonacci series is {}",
        2,
        fibonacci(2)
    );
    println!(
        "The element in the position {} in fibonacci series is {}",
        4,
        fibonacci(4)
    );
    println!(
        "The element in the position {} in fibonacci series is {}",
        22,
        fibonacci(22)
    );
    println!(
        "The element in the position {} in fibonacci series is {}",
        20,
        fibonacci(20)
    );
}
