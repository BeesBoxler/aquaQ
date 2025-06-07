use std::fs::read_to_string;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            'U' => Self::Up,
            'D' => Self::Down,
            'L' => Self::Left,
            'R' => Self::Right,
            _ => return Err(()),
        })
    }
}

#[derive(Debug)]
struct Dice {
    front: u8,
    left: u8,
    top: u8,
}

impl Dice {
    fn new(front: u8, left: u8, top: u8) -> Self {
        Dice { front, left, top }
    }

    fn rotate(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => {
                let bottom = 7 - self.top;
                self.top = self.front;
                self.front = bottom;
            }
            Direction::Down => {
                let back = 7 - self.front;
                self.front = self.top;
                self.top = back;
            }
            Direction::Left => {
                let right = 7 - self.left;
                self.left = self.front;
                self.front = right;
            }
            Direction::Right => {
                let back = 7 - self.front;
                self.front = self.left;
                self.left = back;
            }
        }
    }
}

fn main() {
    let input: Vec<Direction> = read_to_string("input.txt")
        .expect("No input.txt in directory")
        .trim()
        .chars()
        .map(|c| c.try_into().expect("Malformed input"))
        .collect();

    let mut dice_one = Dice::new(1, 2, 3);
    let mut dice_two = Dice::new(1, 3, 2);

    println!(
        "The solution to challenge 5 is: {}",
        find_and_sum_matches(&mut dice_one, &mut dice_two, input)
    );
}

fn find_and_sum_matches(
    dice_one: &mut Dice,
    dice_two: &mut Dice,
    instructions: Vec<Direction>,
) -> usize {
    let dice_one_faces: Vec<u8> = instructions
        .iter()
        .map(|direction| {
            dice_one.rotate(direction);
            dice_one.front
        })
        .collect();

    let dice_two_faces: Vec<u8> = instructions
        .iter()
        .map(|direction| {
            dice_two.rotate(direction);
            dice_two.front
        })
        .collect();

    (0..instructions.len())
        .filter_map(|i| {
            if dice_one_faces[i] == dice_two_faces[i] {
                Some(i)
            } else {
                None
            }
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::find_and_sum_matches;

    #[test]
    fn test_find_and_sum_matches() {
        let input = "LRDLU"
            .chars()
            .map(|c| c.try_into().expect("Malformed input"))
            .collect();

        let mut dice_one = Dice::new(1, 2, 3);
        let mut dice_two = Dice::new(1, 3, 2);

        assert_eq!(find_and_sum_matches(&mut dice_one, &mut dice_two, input), 5)
    }
}
