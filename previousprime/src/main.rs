fn is_prime(num: u64) -> bool {
    let upper_limit: u64 = f64::sqrt(num as f64) as u64 + 1;
    for i in 2..=upper_limit {
        if num % i == 0 {
            return false;
        }
    }
    true
}

pub fn prev_prime(nbr: u64) -> u64 {
    for i in (1..nbr).rev() {
        if is_prime(i) == true {
            return i;
        }
    }
    nbr
}

fn main() {
    println!("The previous prime number before 34 is: {}", prev_prime(34));
}
