use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").expect("No input.txt found");
    println!("The answer to challenge 2 is: {}", challenge(&input));
}

fn challenge(input: &str) -> usize {
    let mut numbers: Vec<usize> = input
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();
    let mut index = 0;
    while index < numbers.len() {
        let curr = numbers[index];
        let mut latest = index;
        for i in (index + 1)..numbers.len() {
            if numbers[i] == curr {
                latest = i
            }
        }
        index += 1;
        if latest > index {
            numbers.drain(index..=latest);
        }
    }

    numbers.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_correct_output() {
        assert_eq!(challenge("1 4 3 2 4 7 2 6 3 6"), 20);
    }
}
