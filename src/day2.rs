#[derive(Clone, Debug, PartialEq)]
pub enum InstructionType {
    FORWARD,
    DOWN,
    UP,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Instruction {
    instruction_type: InstructionType,
    magnitude: i32,
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Instruction> {
    input.lines()
        .map(|l| {
            let split: Vec<&str> = l.split(' ').collect();

            Instruction {
                instruction_type: match split[0] {
                    "forward" => InstructionType::FORWARD,
                    "down" => InstructionType::DOWN,
                    "up" => InstructionType::UP,
                    x => panic!("Bad instruction type: {}", x)
                },
                magnitude: split[1].parse().unwrap(),
            }
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &Vec<Instruction>) -> i32 {
    let mut x = 0;
    let mut y = 0;

    input.iter()
        .for_each(|i| {
            match i.instruction_type {
                InstructionType::FORWARD => x += i.magnitude,
                InstructionType::DOWN => y += i.magnitude,
                InstructionType::UP => y -= i.magnitude,
            };
        });

    x * y
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &Vec<Instruction>) -> i32 {
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;

    input.iter()
        .for_each(|i| {
            match i.instruction_type {
                InstructionType::FORWARD => {
                    x += i.magnitude;
                    y += aim * i.magnitude;
                },
                InstructionType::DOWN => {
                    aim += i.magnitude;
                },
                InstructionType::UP => {
                    aim -= i.magnitude;
                },
            };
        });

    x * y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

        assert_eq!(solve_part1(&input_generator(data)), 150)
    }

    #[test]
    fn test_part2() {
        let data = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

        assert_eq!(solve_part2(&input_generator(data)), 900)
    }
}