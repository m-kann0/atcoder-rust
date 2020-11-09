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
    let mut lamps: Vec<(usize, usize)> = Vec::new();
    const EMPTY: usize = 0;
    const LAMP: usize = 1;
    const BLOCK: usize = 2;
    let mut grid: Vec<Vec<usize>> = vec![vec![EMPTY; w]; h];
    for _ in 0..n {
        let a: usize = iterator.next().unwrap().parse().unwrap();
        let b: usize = iterator.next().unwrap().parse().unwrap();
        let a = a - 1;
        let b = b - 1;
        // lamps.push((a, b));
        grid[a][b] = LAMP;
    }
    for _ in 0..m {
        let c: usize = iterator.next().unwrap().parse().unwrap();
        let d: usize = iterator.next().unwrap().parse().unwrap();
        let c = c - 1;
        let d = d - 1;
        grid[c][d] = BLOCK;
    }

    let mut lighted: Vec<Vec<Vec<bool>>> = vec![vec![vec![false; 4]; w]; h];
    for ch in 0..h {
        lighted[ch][0][0] = grid[ch][0] == LAMP;
        for cw in 1..w {
            if grid[ch][cw] == LAMP {
                lighted[ch][cw][0] = true;
            } else if grid[ch][cw] == BLOCK {
                continue;
            } else {
                lighted[ch][cw][0] = lighted[ch][cw - 1][0];
            }
        }
        lighted[ch][w - 1][1] = grid[ch][w - 1] == LAMP;
        for cw in (0..(w - 1)).rev() {
            if grid[ch][cw] == LAMP {
                lighted[ch][cw][1] = true;
            } else if grid[ch][cw] == BLOCK {
                continue;
            } else {
                lighted[ch][cw][1] = lighted[ch][cw + 1][1];
            }
        }
    }
    for cw in 0..w {
        lighted[0][cw][2] = grid[0][cw] == LAMP;
        for ch in 1..h {
            if grid[ch][cw] == LAMP {
                lighted[ch][cw][2] = true;
            } else if grid[ch][cw] == BLOCK {
                continue;
            } else {
                lighted[ch][cw][2] = lighted[ch - 1][cw][2];
            }
        }
        lighted[h - 1][cw][3] = grid[h - 1][cw] == LAMP;
        for ch in (0..(h - 1)).rev() {
            if grid[ch][cw] == LAMP {
                lighted[ch][cw][3] = true;
            } else if grid[ch][cw] == BLOCK {
                continue;
            } else {
                lighted[ch][cw][3] = lighted[ch + 1][cw][3];
            }
        }
    }

    let mut ans: usize = 0;
    for ch in 0..h {
        for cw in 0..w {
            let mut light = false;
            for i in 0..4 {
                light |= lighted[ch][cw][i];
            }
            if light {
                ans += 1;
            }
        }
    }
    ans.to_string()
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