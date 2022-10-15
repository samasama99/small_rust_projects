use std::{cmp::max, collections::HashMap};

pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    let mut m = i32::MIN;
    for (_, v) in h.iter() {
        m = max(*v, m);
    }
    m
}

fn main() {
    let mut hash = HashMap::new();
    hash.insert("Daniel", 122);
    hash.insert("Ashley", 333);
    hash.insert("Katie", 334);
    hash.insert("Robert", 14);

    println!(
        "The biggest of the elements in the HashMap is {}",
        bigger(hash)
    );
}
