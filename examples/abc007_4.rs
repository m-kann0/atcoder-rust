use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let a: usize = iterator.next().unwrap().parse().unwrap();
    let b: usize = iterator.next().unwrap().parse().unwrap();
    let ans = count(b) - count(a - 1);
    ans.to_string()
}

fn count(n: usize) -> usize {
    let s: Vec<usize> = n.to_string().chars().map(|c| (c as u8 - '0' as u8) as usize).collect();
    let l = s.len();
    // dp[決定した桁数][N未満確定かどうか（確定なら1）][4または9を含むか（含むなら1）]
    let mut dp: Vec<Vec<Vec<usize>>> = vec![vec![vec![0; 2]; 2]; l + 1];
    dp[0][0][0] = 1;
    for i in 0..l {
        for j in 0..2 {
            for k in 0..2 {
                let d_max = if j == 1 { 9 } else { s[i] };
                for d in 0..=d_max {
                    dp[i + 1][(j == 1 || d < d_max) as usize][(k == 1 || d == 4 || d == 9) as usize] += dp[i][j][k];
                }
            }
        }
    }
    dp[l][0][1] + dp[l][1][1]
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"1 9",
            "2"
        ),
        (
            r"40 49",
            "10"
        ),
        (
            r"1 1000",
            "488"
        ),
        (
            r"1 1000000000000000000",
            "981985601490518016"
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