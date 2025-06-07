use std::fs::read_to_string;

fn main() {
    let keys = [
        " ", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz",
    ];
    let input = read_to_string("input.txt").expect("No input.txt found in current directory.");

    let result: String = input
        .lines()
        .map(|line| {
            let (key, repeat) = line.split_once(' ').unwrap();
            let key: usize = key.parse().unwrap();
            let repeat: usize = repeat.parse().unwrap();

            keys[key][(repeat - 1)..=(repeat - 1)].to_string()
        })
        .collect();

    println!("The result for problem 0 is: {result}");
}
