pub fn part_one(input: &str) -> Option<u32> {
    let mut distance = 0;
    let mut depth = 0;
    for line in input.lines() {
        let mut s = line.split_terminator(' ');
        let (direction, amount) = (s.next().expect("bad input"), s.next().expect("bad input"));
        match direction {
            "forward" => {
                distance += amount.parse::<u32>().expect("bad input");
            }
            "down" => {
                depth += amount.parse::<u32>().expect("bad input");
            }
            "up" => {
                depth -= amount.parse::<u32>().expect("bad input");
            }
            _ => unreachable!("uh oh"),
        }
    }
    Some(distance * depth)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut distance = 0;
    let mut depth = 0;
    let mut aim = 0;
    for line in input.lines() {
        let mut s = line.split_terminator(' ');
        let (direction, amount) = (s.next().expect("bad input"), s.next().expect("bad input"));
        let amount = amount.parse::<u32>().expect("bad input");
        match direction {
            "forward" => {
                distance += amount;
                depth += aim * amount;
            }
            "down" => {
                aim += amount;
            }
            "up" => {
                aim -= amount;
            }
            _ => unreachable!("uh oh"),
        }
    }
    Some(distance * depth)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(150));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(900));
    }
}
