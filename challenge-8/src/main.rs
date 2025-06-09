use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").expect("No input.txt in current directory");
    let mut lines = input.trim().lines();

    lines.next();
    let mut supply: Vec<(usize, usize)> = lines
        .map(|line| {
            let mut chunks = line.split(',');
            chunks.next();
            (
                chunks.next().unwrap().parse().unwrap(),
                chunks.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    dbg!(supply);
}
