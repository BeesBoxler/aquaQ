use std::fs::read_to_string;

fn main() {
    let mut input = read_to_string("input.txt")
        .expect("No file named `input.txt` found.")
        .trim()
        .to_string();
    let length = input.len();
    if length % 3 > 0 {
        for _ in (length % 3)..3 {
            input.push('0');
        }
    }

    let sanitised_input: String = input
        .chars()
        .map(|c| if ('a'..='f').contains(&c) { c } else { '0' })
        .collect();

    let chunk_len = sanitised_input.len() / 3;

    let result = format!(
        "#{}{}{}",
        sanitised_input[0..2].to_string(),
        sanitised_input[chunk_len..chunk_len + 2].to_string(),
        sanitised_input[chunk_len * 2..chunk_len * 2 + 2].to_string(),
    );

    println!("The answer to problem 1 is: {result}");
}
