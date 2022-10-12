pub fn reverse_it(v: i32) -> String {
    let a = v.to_string();
    let b: String = a.chars().rev().collect();
    a + &b.trim_matches('-')
}

#[derive(Debug, PartialEq, Eq, Default)]
struct Point(i32, i32);

fn main() {
    let c: Point = Point(10, 20);
    let c2: Point = Point(10, 20);
    if c == c2 {
        println!("{:?} {:?} are {}", c, c2, c == c2);
    }
    println!("{:?}", c);
    println!("{}", reverse_it(123));
    println!("{}", reverse_it(-123));
}
