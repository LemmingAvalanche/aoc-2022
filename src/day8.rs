type Grid = Vec<Vec<(u8, bool)>>;
type Grid2 = Vec<Vec<(u8, u32)>>;

pub fn solve_part1(input: &str) -> usize {
    let width = input.lines().nth(0).unwrap().len();
    let height = input.lines().count();
    let mut grid: Grid = get_grid(input);
    // check left to right
    // Mark edges
    // index iteration recommended here: https://stackoverflow.com/a/49144170/1725151

    // Making multiple grids facilitated debugging while stumbling
    // towards the right solution
    let left_to_right = mark_left_to_right(&grid, height, width);
    let right_to_left = mark_right_to_left(&grid, height, width);
    let top_to_bottom = mark_top_to_bottom(&grid, height, width);
    let bottom_to_top = mark_bottom_to_top(&grid, height, width);
    let result = or_all(
        &left_to_right,
        &right_to_left,
        &top_to_bottom,
        &bottom_to_top,
        height,
        width,
    );
    count_visible(&result)
}

pub fn solve_part2(input: &str) -> u32 {
    let width = input.lines().nth(0).unwrap().len();
    let height = input.lines().count();
    let mut grid: Grid2 = get_grid_p2(input);

    // Making multiple grids facilitated debugging while stumbling
    // towards the right solution
    let left_to_right = mark_left_to_right_p2(&grid, height, width);
    let right_to_left = mark_right_to_left_p2(&grid, height, width);
    let top_to_bottom = mark_top_to_bottom_p2(&grid, height, width);
    let bottom_to_top = mark_bottom_to_top_p2(&grid, height, width);
    let result = multiply_all(
        &left_to_right,
        &right_to_left,
        &top_to_bottom,
        &bottom_to_top,
        height,
        width,
    );
    max(&result, height, width)
}

fn get_grid(input: &str) -> Grid {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .zip(std::iter::repeat(false))
                .collect::<Vec<(u8, bool)>>()
        })
        .collect()
}

fn get_grid_p2(input: &str) -> Grid2 {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .zip(std::iter::repeat(0))
                .collect::<Vec<(u8, u32)>>()
        })
        .collect()
}

fn count_visible(grid: &Grid) -> usize {
    // Not nice-looking
    grid.iter()
        .map(|vec| vec.iter().map(|p| p.1).filter(|b| *b).count())
        .sum::<usize>()
        .try_into()
        .unwrap()
}

fn mark_left_to_right(original: &Grid, height: usize, width: usize) -> Grid {
    let mut grid = original.clone();
    for i in 0..height {
        grid[i][0].1 = true;
    }
    for i in 0..height {
        let mut max_cell = grid[i][0].0;
        for j in 1..width {
            let (value, existing) = grid[i][j];
            let new_cond = value > max_cell;
            grid[i][j].1 = new_cond;

            max_cell = max_cell.max(grid[i][j].0);
        }
    }
    grid
}

fn mark_right_to_left(original: &Grid, height: usize, width: usize) -> Grid {
    let mut grid = original.clone();
    // check right to left
    // Mark
    for i in 0..height {
        grid[i][width - 1].1 = true;
    }
    for i in 0..height {
        let mut max_cell = grid[i][width - 1].0;
        for j in (0..width - 1).rev() {
            let (value, existing) = grid[i][j];
            let new_cond = value > max_cell;
            grid[i][j].1 = new_cond;

            max_cell = max_cell.max(grid[i][j].0);
        }
    }
    grid
}

fn mark_top_to_bottom(original: &Grid, height: usize, width: usize) -> Grid {
    let mut grid = original.clone();
    // check top to bottom
    // Mark
    for j in 0..width {
        grid[0][j].1 = true;
    }

    for j in 0..width {
        let mut max_cell = grid[0][j].0;
        for i in 1..height {
            let (value, existing) = grid[i][j];
            let new_cond = value > max_cell;
            grid[i][j].1 = new_cond;

            max_cell = max_cell.max(grid[i][j].0);
        }
    }
    grid
}

fn mark_bottom_to_top(original: &Grid, height: usize, width: usize) -> Grid {
    let mut grid = original.clone();
    // check bottom to top
    // Mark
    for j in 0..width {
        grid[height - 1][j].1 = true;
    }
    for j in 0..width {
        let mut max_cell = grid[height - 1][j].0;
        for i in (0..height - 1).rev() {
            let (value, existing) = grid[i][j];
            let new_cond = value > max_cell;
            grid[i][j].1 = new_cond;
            max_cell = max_cell.max(grid[i][j].0);
        }
    }
    grid
}

fn or_all(
    grid1: &Grid,
    grid2: &Grid,
    grid3: &Grid,
    grid4: &Grid,
    height: usize,
    width: usize,
) -> Grid {
    let mut grid = grid1.clone();
    let mut count = 0;
    for i in 0..height {
        for j in 0..width {
            grid[i][j].1 = grid1[i][j].1 || grid2[i][j].1 || grid3[i][j].1 || grid4[i][j].1;
        }
    }
    grid
}

fn mark_left_to_right_p2(original: &Grid2, height: usize, width: usize) -> Grid2 {
    let mut grid = original.clone();
    for i in 0..height {
        grid[i][0].1 = 0;
    }
    for i in 0..height {
        for j in 0..width {
            let value = grid[i][j].0;
            let mut scenic = 0;
            for k in j + 1..width {
                let other = grid[i][k].0;
                scenic = scenic + 1;
                if value <= other {
                    break;
                }
            }
            grid[i][j].1 = scenic;
        }
    }
    grid
}

fn mark_right_to_left_p2(original: &Grid2, height: usize, width: usize) -> Grid2 {
    let mut grid = original.clone();
    // check right to left
    // Mark
    for i in 0..height {
        grid[i][width - 1].1 = 0;
    }
    for i in 0..height {
        for j in (0..width).rev() {
            let value = grid[i][j].0;
            let mut scenic = 0;
            let mut k = j.saturating_sub(1);
            for k in (0..j).rev() {
                let other = grid[i][k].0;
                scenic = scenic + 1;
                if value <= other {
                    break;
                }
            }
            grid[i][j].1 = scenic;
        }
    }
    grid
}

fn mark_top_to_bottom_p2(original: &Grid2, height: usize, width: usize) -> Grid2 {
    let mut grid = original.clone();
    // check top to bottom
    // Mark
    for j in 0..width {
        grid[0][j].1 = 0;
    }

    for j in 0..width {
        for i in 0..height {
            let value = grid[i][j].0;
            let mut scenic = 0;
            for k in i + 1..height {
                let other = grid[k][j].0;
                scenic = scenic + 1;
                if value <= other {
                    break;
                }
            }
            grid[i][j].1 = scenic;
        }
    }
    grid
}

fn mark_bottom_to_top_p2(original: &Grid2, height: usize, width: usize) -> Grid2 {
    let mut grid = original.clone();
    // check bottom to top
    // Mark
    for j in 0..width {
        grid[height - 1][j].1 = 0;
    }

    for j in 0..width {
        for i in (0..height).rev() {
            let value = grid[i][j].0;
            let mut scenic = 0;
            for k in (0..i).rev() {
                let other = grid[k][j].0;
                scenic = scenic + 1;
                if value <= other {
                    break;
                }
            }
            grid[i][j].1 = scenic;
        }
    }
    grid
}

fn multiply_all(
    grid1: &Grid2,
    grid2: &Grid2,
    grid3: &Grid2,
    grid4: &Grid2,
    height: usize,
    width: usize,
) -> Grid2 {
    let mut grid = grid1.clone();
    let mut count = 0;
    for i in 0..height {
        for j in 0..width {
            grid[i][j].1 = grid1[i][j].1 * grid2[i][j].1 * grid3[i][j].1 * grid4[i][j].1;
        }
    }
    grid
}

fn max(grid: &Grid2, height: usize, width: usize) -> u32 {
    let mut max = 0;
    for i in 0..height {
        for j in 0..width {
            if grid[i][j].1 > max {
                max = grid[i][j].1;
            }
        }
    }
    max
}
