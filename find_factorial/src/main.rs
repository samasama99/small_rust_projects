pub fn factorial(num: u64) -> u64 {
    if num <= 1 {
        return 1;
    }
    let mut fac = 1;
    for i in 2..=num {
        fac *= i;
    }
    fac
}

fn main() {
    println!("The factorial of 0 = {}", factorial(0));
    println!("The factorial of 1 = {}", factorial(1));
    println!("The factorial of 5 = {}", factorial(5));
    println!("The factorial of 10 = {}", factorial(10));
    println!("The factorial of 19 = {}", factorial(19));
}
