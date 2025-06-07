use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt")
        .expect("No input.txt in directory")
        .trim()
        .parse()
        .unwrap();
    println!(
        "The answer to challenge 4 is: {}",
        calculate_coprime_sum(input)
    );
}

fn calculate_coprime_sum(n: usize) -> usize {
    (1..n).filter(|i| n.is_coprime(*i)).sum()
}

trait Coprime {
    fn is_coprime(&self, other: Self) -> bool;
    fn gcd(&self, other: Self) -> Self;
}

impl Coprime for usize {
    fn is_coprime(&self, other: Self) -> bool {
        self.gcd(other) == 1
    }

    fn gcd(&self, other: Self) -> Self {
        let mut b = other;
        let mut a = *self;
        while b > 0 {
            let n = a;
            a = b;
            b = n % b;
        }

        a
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calculate_coprime_sum() {
        assert_eq!(calculate_coprime_sum(15), 60);
    }

    #[test]
    fn test_is_coprime() {
        assert!(15.is_coprime(13));
    }
}
