use std::env::args;
use std::fs::read_to_string;

fn char_to_grid(c: char) -> Vec<Vec<bool>> {
    fn str_to_grid(s: &str) -> Vec<Vec<bool>> {
        s.split("\n")
            .map(|line| line.chars().map(|c| c == '#').collect())
            .collect()
    }

    match c.to_lowercase().next().unwrap() {
        'a' => str_to_grid(&[
            ".##.",
            "#..#",
            "#..#",
            "####",
            "#..#",
            "#..#"].join("\n")),
        'b' => str_to_grid(&[
            "###.",
            "#..#",
            "###.",
            "#..#",
            "#..#",
            "###."].join("\n")),
        'c' => str_to_grid(&[
            ".##.",
            "#..#",
            "#...",
            "#...",
            "#..#",
            ".##."].join("\n")),
        'd' => str_to_grid(&[
            "###.",
            "#..#",
            "#..#",
            "#..#",
            "#..#",
            "###."].join("\n")),
        'e' => str_to_grid(&[
            "####",
            "#...",
            "###.",
            "#...",
            "#...",
            "####"].join("\n")),
        'f' => str_to_grid(&[
            "####",
            "#...",
            "###.",
            "#...",
            "#...",
            "#..."].join("\n")),
        'g' => str_to_grid(&[
            ".##.",
            "#...",
            "#...",
            "#.##",
            "#..#",
            ".##."].join("\n")),
        'h' => str_to_grid(&[
            "#..#",
            "#..#",
            "####",
            "#..#",
            "#..#",
            "#..#"].join("\n")),
        'i' => str_to_grid(&[
            "#",
            "#",
            "#",
            "#",
            "#",
            "#"].join("\n")),
        'j' => str_to_grid(&[
            "...#",
            "...#",
            "...#",
            "#..#",
            "#..#",
            ".##."].join("\n")),
        'k' => str_to_grid(&[
            "#..#",
            "#.#.",
            "##..",
            "#.#.",
            "#.#.",
            "#..#"].join("\n")),
        'l' => str_to_grid(&[
            "#...",
            "#...",
            "#...",
            "#...",
            "#...",
            "####"].join("\n")),
        'm' => str_to_grid(&[
            "##...##",
            "#.#.#.#",
            "#.#.#.#",
            "#..#..#",
            "#.....#",
            "#.....#"].join("\n")),
        'n' => str_to_grid(&[
            "##...#",
            "#.#..#",
            "#.#..#",
            "#..#.#",
            "#..#.#",
            "#...##"].join("\n")),
        'o' => str_to_grid(&[
            ".##.",
            "#..#",
            "#..#",
            "#..#",
            "#..#",
            ".##."].join("\n")),
        'p' => str_to_grid(&[
            "###.",
            "#..#",
            "###.",
            "#...",
            "#...",
            "#..."].join("\n")),
        'q' => str_to_grid(&[
            ".####.",
            "#....#",
            "#....#",
            "#....#",
            "#...#.",
            ".###.#"].join("\n")),
        'r' => str_to_grid(&[
            "###.",
            "#..#",
            "#..#",
            "###.",
            "#..#",
            "#..#"].join("\n")),
        's' => str_to_grid(&[
            ".###",
            "#...",
            ".##.",
            "...#",
            "...#",
            "###."].join("\n")),
        't' => str_to_grid(&[
            "#####",
            "..#..",
            "..#..",
            "..#..",
            "..#..",
            "..#.."].join("\n")),
        'u' => str_to_grid(&[
            "#..#",
            "#..#",
            "#..#",
            "#..#",
            "#..#",
            ".##."].join("\n")),
        'v' => str_to_grid(&[
            "#...#",
            "#...#",
            "#...#",
            "#...#",
            ".#.#.",
            "..#.."].join("\n")),
        'w' => str_to_grid(&[
            "#.......#",
            "#.......#",
            "#.......#",
            "#...#...#",
            ".#.#.#.#",
            "..#...#.."].join("\n")),
        'x' => str_to_grid(&[
            "#...#",
            ".#.#.",
            "..#..",
            ".#.#.",
            "#...#",
            "#...#"].join("\n")),
        'y' => str_to_grid(&[
            "#...#",
            ".#.#.",
            "..#..",
            "..#..",
            "..#..",
            "..#.."].join("\n")),
        'z' => str_to_grid(&[
            "#####",
            "....#",
            "..##.",
            ".#...",
            "#....",
            "#####"].join("\n")),
        ' ' => str_to_grid(&[
            "...",
            "...",
            "...",
            "...",
            "...",
            "..."].join("\n")),
        '.' => str_to_grid(&[
            "...",
            "...",
            "...",
            "...",
            "...",
            "#.."].join("\n")),
        ',' => str_to_grid(&[
            "...",
            "...",
            "...",
            "...",
            "...",
            ".#.",
            "#.."].join("\n")),
        '?' => str_to_grid(&[
            ".##.",
            "#..#",
            "...#",
            "..#.",
            "....",
            "..#."].join("\n")),
        '!' => str_to_grid(&[
            "..#.",
            "..#.",
            "..#.",
            "..#.",
            "....",
            "..#."].join("\n")),
        '\'' => str_to_grid(&[
            "..#.",
            ".#..",
            "....",
            "....",
            "....",
            "...."].join("\n")),
        '(' => str_to_grid(&[
            ".#.",
            "#..",
            "#..",
            "#..",
            "#..",
            ".#."].join("\n")),
        ')' => str_to_grid(&[
            ".#.",
            "..#",
            "..#",
            "..#",
            "..#",
            ".#."].join("\n")),
        _ => panic!("Unknown input"),
    }
}

fn combine_grids_horizontally(grids: &[Vec<Vec<bool>>]) -> Vec<Vec<bool>> {
    let x_len = grids.iter().fold(0, |acc, grid| acc + 1 + grid[0].len());
    let y_len = grids.iter().max_by_key(|grid| grid.len()).unwrap().len();
    let mut res = vec![vec![false; x_len]; y_len];

    let mut start_x = 0;
    for grid in grids {
        for (y, row) in grid.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                res[y][start_x + x] = cell;
            }
        }

        start_x += grid[0].len() + 1;
    }

    res
}

fn combine_grids_vertically(grids: &[Vec<Vec<bool>>]) -> Vec<Vec<bool>> {
    let y_len = grids.iter().fold(0, |acc, grid| acc + 1 + grid.len());
    let x_len = grids.iter().max_by_key(|grid| grid[0].len()).unwrap()[0].len();
    let mut res = vec![vec![false; x_len]; y_len];

    let mut start_y = 0;
    for grid in grids {
        for (y, row) in grid.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                res[y + start_y][x] = cell;
            }
        }

        start_y += grid.len() + 1;
    }

    res
}

fn split_long_line(max_length: usize, line: String) -> Vec<String> {
    if line.trim().is_empty() {
        // otherwise this wouldn't be included
        return vec![" ".to_string()];
    }

    let mut res = vec![];
    let mut curr_line = String::new();
    for word in line.split(" ") {
        if curr_line.len() + word.len() + 1 > max_length {
            res.push(curr_line);
            curr_line = String::new();
        }

        if !curr_line.is_empty() {
            curr_line.push(' ');
        }

        curr_line.push_str(word);
    }

    if !curr_line.is_empty() {
        res.push(curr_line);
    }

    res

}

const DO_COPY: bool = true;

fn main() {
    // let num_folds = 5;
    let num_folds = args().skip(1).next().unwrap_or("5".to_string()).parse().unwrap();
    // let input = args().skip(2).collect::<Vec<_>>();

    let input = read_to_string("input.txt").unwrap();

    let mut input_lines = vec![];

    for line in input.split("\n") {
        input_lines.extend(split_long_line(26, format!("{} ", line)));
    }


    // let mut ultimate_grid = Vec::new();

    let mut line_grids = Vec::new();
    for input_line in input_lines {
        let grid = combine_grids_horizontally(
            &input_line.chars().map(char_to_grid).collect::<Vec<_>>()
        );

        line_grids.push(grid);
    }

    // now merge all lines
    let grid = combine_grids_vertically(&line_grids);

    // let grid = ultimate_grid;

    let mut positions = grid.iter().enumerate()
        .map(|(y, row)| {
            row.iter().enumerate().filter(|&(_, &cell)| cell).map(|(x, _)| (x, y)).collect::<Vec<_>>()
        }).flatten().collect::<Vec<(usize, usize)>>();


    let mut folds: Vec<(u8, usize)> = vec![];

    for _ in 0..num_folds {
        let mut fold = (0, 0);

        let direction = rand::random::<u8>() % 2;
        fold.0 = direction;
        if direction == 0 {
            // x fold, so copy over to the right
            fold.1 = get_max_x(&positions) + 1;
            // move over like 50% to the other side
            positions = positions.into_iter().map(|(x, y)| {
                if rand::random::<bool>() {
                    /*
                    ...#..
                    should be turned into
                    ......|..#...

                     */

                    let new_x = fold.1 + (fold.1 - x);
                    let mut res = vec![(new_x, y)];
                    if DO_COPY {
                        if rand::random::<u32>() < 100000000 {
                            res.push((x, y));
                        }
                    }
                    res
                } else {
                    vec![(x, y)]
                }
            }).flatten().collect();
        } else {
            // y fold, so copy down
            fold.1 = get_max_y(&positions) + 1;
            // move over like 50% to the other side
            positions = positions.into_iter().map(|(x, y)| {
                if rand::random::<bool>() {
                    let new_y = fold.1 + (fold.1 - y);
                    let mut res = vec![(x, new_y)];
                    if DO_COPY {
                        if rand::random::<u32>() < 100000000 {
                            res.push((x, y));
                        }
                    }
                    res
                } else {
                    vec![(x, y)]
                }
            }).flatten().collect();
        }

        folds.push(fold);
    }

    for &(x, y) in &positions {
        println!("{},{}", x, y);
    }
    println!();
    for &(dir, fold_pos) in folds.iter().rev() {
        println!("fold along {}={}", if dir == 0 { "x" } else { "y" }, fold_pos);
    }
}

fn get_max_x(pos: &Vec<(usize, usize)>) -> usize {
    pos.iter().map(|&(x, _)| x).max().unwrap()
}

fn get_max_y(pos: &Vec<(usize, usize)>) -> usize {
    pos.iter().map(|&(_, y)| y).max().unwrap()
}
