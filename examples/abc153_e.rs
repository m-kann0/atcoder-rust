use std::io::Read;
use std::cmp::{max, min};

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let h: usize = iterator.next().unwrap().parse().unwrap();
    let n: usize = iterator.next().unwrap().parse().unwrap();
    let mut a: Vec<usize> = Vec::new();
    let mut b: Vec<usize> = Vec::new();
    for _ in 0..n {
        a.push(iterator.next().unwrap().parse().unwrap());
        b.push(iterator.next().unwrap().parse().unwrap());
    }

    const INF: usize = 1_000_000_000;
    let mut dp: Vec<usize> = vec![INF; h + 1];
    dp[0] = 0;
    for i in 0..h {
        for j in 0..n {
            let k = min(h, i + a[j]);
            dp[k] = min(dp[k], dp[i] + b[j]);
        }
    }

    return dp[h].to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"9 3
8 3
4 2
2 1",
            "4"
        ),
        (
            r"100 6
1 1
2 3
3 9
4 27
5 81
6 243",
            "100"
        ),
        (
            r"9999 10
540 7550
691 9680
700 9790
510 7150
415 5818
551 7712
587 8227
619 8671
588 8228
176 2461",
            "139815"
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