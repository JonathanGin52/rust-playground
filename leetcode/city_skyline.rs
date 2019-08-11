use std::cmp;

fn main() {
    let grid = vec![
        vec![3, 0, 8, 4],
        vec![2, 4, 5, 7],
        vec![9, 2, 6, 3],
        vec![0, 3, 1, 0]
    ];
    assert_eq!(max_increase_keeping_skyline(grid), 35);
}

pub fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
    let mut sum: i32 = 0;
    let n = grid.len();
    let mut horizontal_skyline: Vec<i32> = vec![0; n];
    let mut vertical_skyline: Vec<i32> = vec![0; n];

    for (x, row) in grid.iter().enumerate() {
        for (y, col) in row.iter().enumerate() {
            horizontal_skyline[x] = cmp::max(horizontal_skyline[x], *col);
            vertical_skyline[y] = cmp::max(vertical_skyline[y], *col);
        }
    }
    
    for x in 0..n {
        for y in 0..n {
            sum += cmp::min(horizontal_skyline[x], vertical_skyline[y]) - grid[x][y];
        }
    }

    sum
}
