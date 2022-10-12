fn inv_pyramid(v: String, i: usize) -> Vec<String> {
    let mut res = Vec::new();
    for i in 1..i + 1 {
        res.push(" ".repeat(i) + &v.repeat(i));
    }
    let mut rev_res = res.clone();
    rev_res.pop().unwrap();
    rev_res.reverse();
    res.append(&mut rev_res);
    res
}

fn main() {
    let a = inv_pyramid(String::from("#"), 1);
    let b = inv_pyramid(String::from("a"), 2);
    let c = inv_pyramid(String::from(">"), 5);
    let d = inv_pyramid(String::from("&"), 8);

    for v in a.iter() {
        println!("{:?}", v);
    }
    for v in b.iter() {
        println!("{:?}", v);
    }
    for v in c.iter() {
        println!("{:?}", v);
    }
    for v in d.iter() {
        println!("{:?}", v);
    }
}
