use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let input = read_to_string("input.txt").expect("No input.txt in directory");
    let mut chunks = input.split_whitespace();
    let number: usize = chunks
        .last()
        .expect("Malformed input")
        .parse()
        .expect("Malformed Input");

    println!(
        "The answer to challenge 6 is: {}",
        get_count_of_ones(number)
    );
}

fn get_summable_components(total: usize) -> Vec<Vec<usize>> {
    let mut result = HashSet::new();

    (0..=total).for_each(|i| {
        (0..=total - i).for_each(|j| {
            if i + j <= total {
                (0..=total - (i + j)).for_each(|k| {
                    if i + j + k <= total {
                        result.insert(vec![i, j, total - (j + i)]);
                    }
                })
            }
        })
    });

    result.iter().cloned().collect()
}

fn get_count_of_ones(input: usize) -> usize {
    let summable_components = get_summable_components(input);
    let one_byte = 49u8;

    format!("{:?}", summable_components)
        .bytes()
        .filter(|b| b == &one_byte)
        .count()
}
