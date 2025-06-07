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
struct Point(usize, usize);

struct Field {
    map: Vec<Vec<char>>,
    position: Point,
}

impl Field {
    fn new(map: Vec<Vec<char>>, origin: Point) -> Self {
        Field {
            map,
            position: origin,
        }
    }

    fn traverse(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => {
                let new = self.position.0.max(1) - 1;
                if self.map[new][self.position.1] == '#' {
                    self.position.0 = new;
                }
            }
            Direction::Down => {
                let new = self.position.0.min(self.map.len() - 2) + 1;
                if self.map[new][self.position.1] == '#' {
                    self.position.0 = new;
                }
            }
            Direction::Left => {
                let new = self.position.1.max(1) - 1;
                if self.map[self.position.0][new] == '#' {
                    self.position.1 = new;
                }
            }
            Direction::Right => {
                let new = self.position.1.min(self.map[self.position.0].len() - 2) + 1;
                if self.map[self.position.0][new] == '#' {
                    self.position.1 = new;
                }
            }
        };
    }
}

fn main() {
    let input = read_to_string("input.txt").expect("No input file in directory");
    println!(
        "The solution to challenge 3 is: {}",
        calculate_positions(&input.trim())
    );
}

fn calculate_positions(input: &str) -> usize {
    let map = vec![
        vec![' ', ' ', '#', '#', ' ', ' '],
        vec![' ', '#', '#', '#', '#', ' '],
        vec!['#', '#', '#', '#', '#', '#'],
        vec!['#', '#', '#', '#', '#', '#'],
        vec![' ', '#', '#', '#', '#', ' '],
        vec![' ', ' ', '#', '#', ' ', ' '],
    ];
    let mut field = Field::new(map, Point(0, 2));
    let input: Vec<Direction> = input.chars().map(|c| c.try_into().unwrap()).collect();

    input
        .iter()
        .map(|direction| {
            field.traverse(direction);
            field.position.0 + field.position.1
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calculate_position() {
        let input = "UDRR";

        assert_eq!(calculate_positions(input), 14);
    }
}
