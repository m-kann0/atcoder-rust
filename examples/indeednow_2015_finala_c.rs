use std::io::Read;
use std::cmp::max;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let m: usize = iterator.next().unwrap().parse().unwrap();
    let mut dp: Vec<Vec<Vec<usize>>> = vec![vec![vec![0; 101]; 101]; 101];
    for _ in 0..n {
        let a: usize = iterator.next().unwrap().parse().unwrap();
        let b: usize = iterator.next().unwrap().parse().unwrap();
        let c: usize = iterator.next().unwrap().parse().unwrap();
        let w: usize = iterator.next().unwrap().parse().unwrap();
        dp[a][b][c] = max(dp[a][b][c], w);
    }
    for i in 0..=100 {
        for j in 0..=100 {
            for k in 0..=100 {
                dp[i][j][k] = max(
                    max(
                        dp[i][j][k],
                        if i > 0 { dp[i - 1][j][k] } else { 0 }
                    ),
                    max(
                        if j > 0 { dp[i][j - 1][k] } else { 0 },
                        if k > 0 { dp[i][j][k - 1] } else { 0 }
                    )
                );
            }
        }
    }
    let mut result = String::new();
    for _ in 0..m {
        let x: usize = iterator.next().unwrap().parse().unwrap();
        let y: usize = iterator.next().unwrap().parse().unwrap();
        let z: usize = iterator.next().unwrap().parse().unwrap();
        result.push_str(&format!("{}\n", dp[x][y][z]));
    }
    result.trim().to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 6
1 2 3 3
3 3 3 6
4 4 4 8
3 4 3
4 4 4
100 100 1
2 3 4
0 0 0
100 100 100",
            "6
8
0
3
0
8"
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