use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let h: usize = iterator.next().unwrap().parse().unwrap();
    let w: usize = iterator.next().unwrap().parse().unwrap();
    let n: usize = iterator.next().unwrap().parse().unwrap();
    let m: usize = iterator.next().unwrap().parse().unwrap();
    const EMPTY: usize = 0;
    const LIGHT: usize = 1;
    const BLOCK: usize = 2;
    let mut grid = vec![vec![EMPTY; w]; h];
    for _ in 0..n {
        let a: usize = iterator.next().unwrap().parse().unwrap();
        let b: usize = iterator.next().unwrap().parse().unwrap();
        grid[a - 1][b - 1] = LIGHT;
    }
    for _ in 0..m {
        let c: usize = iterator.next().unwrap().parse().unwrap();
        let d: usize = iterator.next().unwrap().parse().unwrap();
        grid[c - 1][d - 1] = BLOCK;
    }

    let mut result = vec![vec![false; w]; h];
    for _ in 0..4 {
        let h = result.len();
        let w = result[0].len();
        let mut now = vec![vec![false; w]; h];

        for i in 0..h {
            for j in 0..w {
                match grid[i][j] {
                    EMPTY => {
                        if j > 0 {
                            now[i][j] = now[i][j - 1]
                        }
                    },
                    LIGHT => {
                        now[i][j] = true;
                    },
                    BLOCK => {
                        now[i][j] = false;
                    },
                    _ => panic!()
                }
                result[i][j] |= now[i][j];
            }
        }

        grid = grid::rotate_right(&grid);
        result = grid::rotate_right(&result);
    }

    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            if result[i][j] {
                ans += 1;
            }
        }
    }
    ans.to_string()
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

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 3 2 1
1 1
2 3
2 2",
            "7"
        ),
        (
            r"4 4 3 3
1 2
1 3
3 4
2 3
2 4
3 2",
            "8"
        ),
        (
            r"5 5 5 1
1 1
2 2
3 3
4 4
5 5
4 2",
            "24"
        ),
    ];

    let mut all_ok = true;
    for (i, case) in cases.iter().enumerate() {
        print!("case {} : ", i);

        let expected = case.1;
        let actual = solve(case.0);

        if expected == actual {
            println!("OK");
        } else {
            println!("NG");
            println!("    Expected: {}", expected);
            println!("    Actual  : {}", actual);

            all_ok = false;
        }
    }

    assert_eq!(all_ok, true);
}