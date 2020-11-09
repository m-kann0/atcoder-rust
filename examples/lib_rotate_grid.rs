use std::fmt::Display;

fn main() {
    let grid = vec![
        vec![1, 2, 3, 4],
        vec![5, 6, 7, 8],
        vec![9, 10, 11, 12],
    ];
    debug(&grid);
    println!("---");

    let grid = grid::rotate_right(&grid);
    debug(&grid);
    println!("---");

    let grid = grid::rotate_right(&grid);
    debug(&grid);
    println!("---");

    let grid = grid::rotate_right(&grid);
    debug(&grid);
    println!("---");
}

fn debug<T: Display>(grid: &Vec<Vec<T>>) {
    for line in grid {
        for e in line {
            print!("{:3}", e);
        }
        println!();
    }
}

pub mod grid {
    pub fn rotate_right<T: Clone>(grid: &Vec<Vec<T>>) -> Vec<Vec<T>> {
        if grid.is_empty() {
            return Vec::new();
        }
        let h = grid.len();
        let w = grid[0].len();
        let mut result = Vec::with_capacity(w);
        for j in 0..w {
            let mut line = Vec::with_capacity(h);
            for i in (0..h).rev() {
                line.push(grid[i][j].clone());
            }
            result.push(line);
        }
        result
    }
}
