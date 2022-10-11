use regex::Regex;

fn is_uppercase(s: &String) -> bool {
    s.eq(&s.clone().to_uppercase())
}

// pub fn talking(text: &str) -> &str {
//     if text.len() == 0 {
//         return "Just say something!";
//     }

//     let is_question = Regex::new(".\\?").unwrap();
//     let is_exp = Regex::new(".!").unwrap();

//     if is_uppercase(&text.to_string()) {
//         if is_exp.is_match(text) {
//             return "There is no need to yell, calm down!";
//         }
//         if is_question.is_match(text) {
//             return "Quiet, I am thinking!";
//         }
//     }

//     if is_question.is_match(text) {
//         return "Sure";
//     }

//     return "Interesting";
// }

pub fn talking(text: &str) -> &str {
    let is_question = Regex::new(".\\?").unwrap();
    let is_exp = Regex::new(".!").unwrap();
    match text {
        text if text.len() == 0 => "Just say something!",
        text if is_uppercase(&text.to_string()) => match text {
            text if is_exp.is_match(text) => "There is no need to yell, calm down!",
            text if is_question.is_match(text) => "Quiet, I am thinking!",
            _ => "Interesting",
        },
        text if is_question.is_match(text) => "Sure.",
        _ => "Interesting",
    }
}

fn main() {
    println!("{:?}", talking("JUST DO IT!"));
    println!("{:?}", talking("Hello how are you?"));
    println!("{:?}", talking("WHAT'S GOING ON?"));
    println!("{:?}", talking("something"));
    println!("{:?}", talking(""));
}
