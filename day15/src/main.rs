fn solve1() -> usize {
    include_str!("../input.txt")
        .split(",")
        .map(|command| {
            command
                .chars()
                .filter(|c| *c != '\n')
                .fold(0, |acc, c| ((acc + c as usize) * 17) % 256)
        })
        .sum()
}

fn main() {
    dbg!(solve1());
}
