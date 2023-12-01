const DIGITS: &[&str] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn get_first_digit(line: &str) -> u32 {
    for (idx, _) in line.char_indices() {
        let mut line = line[0..idx + 1].to_string();
        for (num, str_repr) in DIGITS.iter().enumerate() {
            line = line.replace(str_repr, &(num + 1).to_string());
            if let Some(digit) = line
                .chars()
                .skip(line.len() - 1)
                .next()
                .unwrap()
                .to_digit(10)
            {
                return digit;
            }
        }
    }
    return 0;
}

fn get_last_digit(line: &str) -> u32 {
    for (idx, _) in line.char_indices() {
        let mut line = line[(line.len() - idx - 1)..line.len()].to_string();
        for (num, str_repr) in DIGITS.iter().enumerate() {
            line = line.replace(str_repr, &(num + 1).to_string());
            if let Some(digit) = line.chars().next().unwrap().to_digit(10) {
                return digit;
            }
        }
    }
    return 0;
}

fn solve2() {
    let input = include_str!("../input.txt");

    let res2 = input
        .lines()
        .map(|line| {
            let first = get_first_digit(line);
            let last = get_last_digit(line);

            return first * 10 + last;
        })
        .sum::<u32>();

    dbg!(res2);
}

fn solve1() {
    let input = include_str!("../input.txt");

    let res1 = input
        .lines()
        .map(|line| {
            let numbers: Vec<_> = line.chars().filter_map(|char| char.to_digit(10)).collect();

            let first = numbers.first().unwrap();
            let last = numbers.last().unwrap();

            return first * 10 + last;
        })
        .sum::<u32>();

    dbg!(res1);
}

fn main() {
    solve1();
    solve2();
}
