use std::io::Read;
use std::cmp::min;

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
    let mut map: Vec<Vec<char>> = Vec::new();
    for _ in 0..h {
        let line: Vec<char> = iterator.next().unwrap().chars().collect();
        map.push(line);
    }

    let mut dp: Vec<Vec<usize>> = vec![vec![0; w]; h];
    if map[0][0] == '#' {
        dp[0][0] = 1;
    }
    const INF: usize = 100_001;
    for i in 0..h {
        for j in 0..w {
            if i == 0 && j == 0 {
                continue;
            }
            let mut d = INF;
            if i > 0 {
                let mut c = dp[i - 1][j];
                if map[i - 1][j] == '.' && map[i][j] == '#' {
                    c += 1;
                }
                d = min(d, c);
            }
            if j > 0 {
                let mut c = dp[i][j - 1];
                if map[i][j - 1] == '.' && map[i][j] == '#' {
                    c += 1;
                }
                d = min(d, c);
            }
            dp[i][j] = d;
        }
    }
    return dp[h - 1][w - 1].to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 3
.##
.#.
##.",
            "1"
        ),
        (
            r"2 2
#.
.#",
            "2"
        ),
        (
            r"4 4
..##
#...
###.
###.",
            "0"
        ),
        (
            r"5 5
.#.#.
#.#.#
.#.#.
#.#.#
.#.#.",
            "4"
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