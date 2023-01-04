use std::char;
use std::collections::HashMap;

fn highest_freq(input: &str) -> char {
    let mut freq: HashMap<char, u32> = HashMap::new();

    for c in input.chars() {
        let count = freq.entry(c).or_default();
        *count += 1;
    }

    let max_pair = freq.iter()
    .max_by(|a, b| a.1.cmp(b.1));

    match max_pair {
        Some((c, _)) => return *c,
        None => return ' ',
    }
}

fn main() {
    let most_common = highest_freq("aaabbcc");
    println!("{}", most_common);
}
