#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &Vec<u32>) -> u32 {
    let mut num_increases = 0;
    for (i, current) in input.iter().enumerate().skip(1) {
        if let Some(previous) = input.get(i - 1) {
            if current > previous {
                num_increases += 1;
            }
        }
    }

    num_increases
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &Vec<u32>) -> u32 {
    let mut num_increases = 0;

    let mut previous_sum = u32::MAX;
    for window in input.windows(3) {
        let sum = window.iter().sum();

        if sum > previous_sum {
            num_increases += 1;
        }

        previous_sum = sum;
    }

    num_increases
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        let data = "199
200
208
210
200
207
240
269
260
263";

        assert_eq!(solve_part1(&input_generator(data)), 7)
    }

    #[test]
    fn sample_02() {
        let data = "199
200
208
210
200
207
240
269
260
263";

        assert_eq!(solve_part2(&input_generator(data)), 5)
    }
}