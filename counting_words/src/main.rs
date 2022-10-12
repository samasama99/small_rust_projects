use std::collections::HashMap;

fn counting_words(words: &str) -> HashMap<String, u32> {
    let mut counts = HashMap::<String, u32>::new();
    let words = words.to_lowercase();
    for word in words
        .split(&[' ', ',', ':', '_', '.', '\n', '\"', '-', '”', '“', '―'])
        .filter(|w| !w.is_empty())
    {
        *counts.entry(word.to_string()).or_insert(0) += 1;
    }
    counts
}

fn main() {
    println!("{:#?}", counting_words("Hello, world!"));
    println!("{:#?}", counting_words("“Two things are infinite: the universe and human stupidity; and I'm not sure about the universe.”
    ― Albert Einstein "));
    println!("{:#?}", counting_words("Batman, BATMAN, batman, Stop stop"));
}
