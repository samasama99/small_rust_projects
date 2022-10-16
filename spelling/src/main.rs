fn split_num(n: u64) -> Vec<String> {
    let mut res = Vec::new();
    for (index, num) in n.to_string().chars().rev().enumerate() {
        let zeros: String = "0".repeat(index);
        let num = num.to_string() + &zeros;
        res.push(num);
    }
    res.into_iter()
        .filter(|w| *w != "0")
        .collect::<Vec<String>>()
}

fn generate_num(n: &str) -> String {
    match n {
        "1" => "one".to_string(),
        "2" => "two".to_string(),
        "3" => "three".to_string(),
        "4" => "four".to_string(),
        "5" => "five".to_string(),
        "6" => "six".to_string(),
        "7" => "seven".to_string(),
        "8" => "eight".to_string(),
        "9" => "nine".to_string(),
        "10" => "ten".to_string(),
        "11" => "eleven".to_string(),
        "12" => "twelve".to_string(),
        "13" => "thirteen".to_string(),
        "14" => "fourteen".to_string(),
        "15" => "fifteen".to_string(),
        "16" => "sixteen".to_string(),
        "17" => "seventeen".to_string(),
        "18" => "eighteen".to_string(),
        "19" => "nineteen".to_string(),
        "20" => "twenty".to_string(),
        "30" => "thirty".to_string(),
        "40" => "forty".to_string(),
        "50" => "fifty".to_string(),
        "60" => "sixty".to_string(),
        "70" => "seventy".to_string(),
        "80" => "eighty".to_string(),
        "90" => "ninety".to_string(),
        "100" => "hundred".to_string(),
        "1000" => "thousand ".to_string(),
        "1000000" => "million".to_string(),
        _ => unreachable!(),
    }
}

// pub fn spell_helper(n: String, res: String) -> String {
// }

pub fn spell(n: u64) -> String {
    let mut res = Vec::new();
    for num in split_num(n) {
        res.push(generate_num(&num));
    }
    res.join(" ")
}

fn main() {
    println!("{}", spell(1));
    println!("{}", spell(10));
    println!("{}", spell(348));
    println!("{}", spell(9996));
}
