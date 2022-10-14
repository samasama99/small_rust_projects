pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    (c, (c as f64).exp(), (c.abs() as f64).log2())
}

pub fn str_function(a: String) -> (String, String) {
    (
        a.clone(),
        a.split_whitespace()
            .map(|s| s.parse::<f64>().unwrap().exp().to_string())
            .collect::<Vec<String>>()
            .join(" "),
    )
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    (
        b.clone(),
        b.iter().map(|n| (n.abs() as f64).log2()).collect(),
    )
}

fn main() {
    let a: i32 = 0;
    let b = String::from("1 2 4 5 6");
    let c = vec![1, 2, 4, 5];

    println!("{:?}", nbr_function(a));
    println!("{:?}", str_function(b));
    println!("{:?}", vec_function(c));
}
