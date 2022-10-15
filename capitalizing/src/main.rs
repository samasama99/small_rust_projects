pub fn capitalize_first(input: &str) -> String {
    match input.len() {
        0 => String::new(),
        1 => input.to_uppercase(),
        _ => input[0..1].to_uppercase() + &input[1..].to_string(),
    }
}

pub fn title_case(input: &str) -> String {
    let mut res = Vec::new();
    for s in input.split_whitespace() {
        res.push(capitalize_first(s));
    }
    res.join(" ")
}

pub fn change_case(input: &str) -> String {
    let mut res = String::new();
    for c in input.chars() {
        match c.is_lowercase() {
            true => res.push_str(&c.to_uppercase().to_string()),
            false => res.push_str(&c.to_lowercase().to_string()),
        }
    }
    res
}

fn main() {
    println!("{}", capitalize_first("joe is missing"));
    println!("{}", title_case("jill is leaving A"));
    println!("{}", change_case("heLLo THere"));
}
