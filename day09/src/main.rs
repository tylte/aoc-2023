fn solve1() -> isize {
    fn solve_next_value(line: &str) -> isize {
        let mut numbers = line
            .split_whitespace()
            .map(|nb| nb.parse::<isize>().unwrap())
            .collect::<Vec<_>>();

        let mut sum = 0;
        let mut last_number = numbers[numbers.len() - 1];
        while last_number != 0 {
            sum += last_number;
            numbers = numbers
                .windows(2)
                .map(|window| window[1] - window[0])
                .collect();
            last_number = numbers[numbers.len() - 1];
        }

        sum
    }

    let input = include_str!("../input.txt");

    input.lines().map(solve_next_value).sum()
}

fn solve2() -> isize {
    fn solve_previous_value(line: &str) -> isize {
        let mut numbers = line
            .split_whitespace()
            .map(|nb| nb.parse::<isize>().unwrap())
            .collect::<Vec<_>>();

        let mut stack = vec![];
        let mut first_number = numbers[0];
        while numbers[numbers.len() - 1] != 0 {
            stack.push(first_number);
            numbers = numbers
                .windows(2)
                .map(|window| window[1] - window[0])
                .collect();
            first_number = numbers[0];
        }

        stack.into_iter().rev().fold(0, |acc, x| x - acc)
    }

    let input = include_str!("../input.txt");

    input.lines().map(solve_previous_value).sum()
}

fn main() {
    dbg!(solve1());
    dbg!(solve2());
}
