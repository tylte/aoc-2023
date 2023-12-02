fn solve1() -> usize {
    const COLOR_RED: &str = "red";
    const COLOR_GREEN: &str = "green";
    const COLOR_BLUE: &str = "blue";

    const MAX_RED: usize = 12;
    const MAX_GREEN: usize = 13;
    const MAX_BLUE: usize = 14;

    fn is_possible_draw(game: &str) -> bool {
        game.split_once(":")
            .unwrap()
            .1
            .split(";")
            .all(|single_draw| {
                single_draw.split(",").all(|num_and_color| {
                    let (num, color) = num_and_color.trim().split_once(" ").unwrap();
                    let num: usize = num.parse().unwrap();
                    match color {
                        COLOR_RED => num <= MAX_RED,
                        COLOR_GREEN => num <= MAX_GREEN,
                        COLOR_BLUE => num <= MAX_BLUE,
                        _ => panic!("What is this color !? {color}"),
                    }
                })
            })
    }

    let input = include_str!("../input.txt");
    let res: usize = input
        .lines()
        .enumerate()
        .filter_map(|(idx, line)| {
            if is_possible_draw(line) {
                Some(idx + 1)
            } else {
                None
            }
        })
        .sum();

    res
}

fn solve2() -> usize {
    const COLOR_RED: &str = "red";
    const COLOR_GREEN: &str = "green";
    const COLOR_BLUE: &str = "blue";

    const RED_IDX: usize = 0;
    const GREEN_IDX: usize = 1;
    const BLUE_IDX: usize = 2;

    fn game_power(game: &str) -> usize {
        let mut cubes_occ = [0; 3];

        game.split_once(":")
            .unwrap()
            .1
            .split(";")
            .for_each(|single_draw| {
                single_draw.split(",").for_each(|num_and_color| {
                    let (num, color) = num_and_color.trim().split_once(" ").unwrap();
                    let num: usize = num.parse().unwrap();
                    match color {
                        COLOR_RED => cubes_occ[RED_IDX] = cubes_occ[RED_IDX].max(num),
                        COLOR_GREEN => cubes_occ[GREEN_IDX] = cubes_occ[GREEN_IDX].max(num),
                        COLOR_BLUE => cubes_occ[BLUE_IDX] = cubes_occ[BLUE_IDX].max(num),
                        _ => panic!("What is this color !? {color}"),
                    }
                })
            });

        cubes_occ.into_iter().product()
    }

    let input = include_str!("../input.txt");
    let res: usize = input.lines().map(game_power).sum();

    res
}

fn main() {
    dbg!(solve1());
    dbg!(solve2());
}
