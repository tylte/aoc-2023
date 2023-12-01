fn solve1() {
    let input = include_str!("../input.txt");

    let res = input
        .lines()
        .map(|line| {
            let numbers: Vec<_> = line.chars().filter_map(|char| char.to_digit(10)).collect();

            let first = numbers.first().unwrap();
            let last = numbers.last().unwrap();

            return first * 10 + last;
        })
        .sum::<u32>();

    dbg!(res);
}
fn main() {
    solve1();
}
