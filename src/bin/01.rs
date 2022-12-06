use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .tuple_windows()
            .map(|(a, b)| {
                a.parse::<u32>().expect("bad input") < b.parse::<u32>().expect("bad input")
            })
            .filter(|v| *v)
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .tuple_windows()
            .map(|items: (&str, &str, &str)| {
                [items.0, items.1, items.2]
                    .iter()
                    .map(|item| item.parse::<u32>().expect("bad input"))
                    .sum::<u32>()
            })
            .tuple_windows()
            .map(|(a, b)| a < b)
            .filter(|v| *v)
            .count() as u32,
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(5));
    }
}
