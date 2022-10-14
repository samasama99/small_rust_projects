fn is_prime(num: u64) -> bool {
    let upper_limit: u64 = f64::sqrt(num as f64) as u64 + 1;
    for i in 2..=upper_limit {
        if num % i == 0 {
            return false;
        }
    }
    true
}

pub fn next_prime(nbr: u64) -> u64 {
    for i in nbr.. {
        if is_prime(i) == true {
            return i;
        }
    }
    panic!("should reach here");
}

fn main() {
    println!("The next prime after 4 is: {}", next_prime(4));
    println!("The next prime after 11 is: {}", next_prime(11));
    println!("The next prime after 14 is: {}", next_prime(14));
    println!("The next prime after 22 is: {}", next_prime(22));
}

