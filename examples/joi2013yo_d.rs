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

    let d: usize = iterator.next().unwrap().parse().unwrap();
    let n: usize = iterator.next().unwrap().parse().unwrap();
    let t: Vec<usize> = (0..d).map(|_| iterator.next().unwrap().parse().unwrap()).collect();
    let mut a: Vec<usize> = Vec::new();
    let mut b: Vec<usize> = Vec::new();
    let mut c: Vec<isize> = Vec::new();
    for _ in 0..n {
        let ai: usize = iterator.next().unwrap().parse().unwrap();
        let bi: usize = iterator.next().unwrap().parse().unwrap();
        let ci: isize = iterator.next().unwrap().parse().unwrap();
        a.push(ai);
        b.push(bi);
        c.push(ci);
    }

    let mut dp: Vec<Vec<isize>> = vec![vec![0; n]; d + 1];
    for i in 1..=d {
        for j in 0..n {
            if t[i - 1] < a[j] || b[j] < t[i - 1] {
                dp[i][j] = -1;
                continue;
            }
            if i == 1 {
                dp[i][j] = 0;
                continue;
            }
            let mut mx = -1;
            for k in 0..n {
                if dp[i - 1][k] == -1 {
                    continue;
                }
                mx = max(mx, dp[i - 1][k] + (c[j] - c[k]).abs());
            }
            dp[i][j] = mx;
        }
    }

    // for i in 0..=d {
    //     eprintln!("dp[i] = {:?}", dp[i]);
    // }

    let mut ans = 0;
    for j in 0..n {
        ans = max(ans, dp[d][j]);
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 4
31
27
35
20 25 30
23 29 90
21 35 60
28 33 40",
            "80"
        ),
        (
            r"5 2
26
28
32
29
34
30 35 0
25 30 100",
            "300"
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