use std::fs;

fn scenic_score_top(grid: &[Vec<u8>], i: usize, j: usize, elem: u8) -> (bool, usize) {
    for k in 1..i + 1 {
        if grid[i - k][j] >= elem {
            return (false, k);
        }
    }
    (true, i)
}

fn scenic_score_left(grid: &[Vec<u8>], i: usize, j: usize, elem: u8) -> (bool, usize) {
    for k in 1..j + 1 {
        if grid[i][j - k] >= elem {
            return (false, k);
        }
    }
    (true, j)
}

fn scenic_score_right(grid: &[Vec<u8>], i: usize, j: usize, elem: u8) -> (bool, usize) {
    let width = grid[i].len();

    for k in j + 1..width {
        if grid[i][k] >= elem {
            return (false, k - j);
        }
    }
    (true, width - j - 1)
}

fn scenic_score_bottom(grid: &[Vec<u8>], i: usize, j: usize, elem: u8) -> (bool, usize) {
    let height = grid.len();
    for k in i + 1..height {
        if grid[k][j] >= elem {
            return (false, k - i);
        }
    }
    (true, height - i - 1)
}

fn calc_tree_visible(l: bool, r: bool, b: bool, t: bool) -> usize {
    if t || b || l || r {
        1
    } else {
        0
    }
}

fn main() -> std::io::Result<()> {
    let content = fs::read_to_string("input-08.txt")?;
    let lines = content.split("\r\n");

    let grid = lines
        .map(|l| l.chars().map(|c| c as u8 - 0x30).collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>();

    let width = grid[0].len();
    let height = grid.len();

    let trees = (1..height - 1)
        .flat_map(|x| {
            (1..height - 1)
                .map(|y| {
                    let elem = grid[x][y];

                    let (visible_top, ss_top) = scenic_score_top(&grid, x, y, elem);
                    let (visible_left, ss_left) = scenic_score_left(&grid, x, y, elem);
                    let (visible_right, ss_right) = scenic_score_right(&grid, x, y, elem);
                    let (visible_bottom, ss_bottom) = scenic_score_bottom(&grid, x, y, elem);

                    let visible_trees =
                        calc_tree_visible(visible_left, visible_right, visible_bottom, visible_top);

                    (visible_trees, ss_left * ss_right * ss_bottom * ss_top)
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let part1 = trees.iter().map(|x| x.0).sum::<usize>() + width * 2 + (height - 2) * 2;
    let part2 = trees.iter().max_by_key(|x| x.1).map(|x| x.1).unwrap();

    println!("Part1 - {:?}", part1);
    println!("Part2 - {:?}", part2);

    Ok(())
}
