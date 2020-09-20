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
    let k: usize = iterator.next().unwrap().parse().unwrap();
    let mut l: Vec<usize> = Vec::new();
    let mut r: Vec<usize> = Vec::new();
    for _ in 0..k {
        let li: usize = iterator.next().unwrap().parse().unwrap();
        let ri: usize = iterator.next().unwrap().parse().unwrap();
        l.push(li);
        r.push(ri);
    }

    const MOD: usize = 998244353;

    let mut dp: Vec<usize> = vec![0; n + 1];
    let mut dpsum: Vec<usize> = vec![0; n + 1];
    dp[1] = 1;
    dpsum[1] = 1;
    for i in 2..=n {
        for j in 0..k {
            let li = i as isize - r[j] as isize;
            let ri = i as isize - l[j] as isize;
            if ri < 0 {
                continue;
            }
            let li = max(li, 1) as usize;
            let ri = ri as usize;
            dp[i] = if (dp[i] + dpsum[ri]) % MOD >= dpsum[li - 1] % MOD {
                (dp[i] + dpsum[ri]) % MOD - dpsum[li - 1] % MOD
            } else {
                MOD - (dpsum[li - 1] % MOD - (dp[i] + dpsum[ri]) % MOD)
            };
        }
        dpsum[i] = (dpsum[i - 1] + dp[i]) % MOD;
    }
    dp[n].to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5 2
1 1
3 4",
            "4"
        ),
        (
            r"5 2
3 3
5 5",
            "0"
        ),
        (
            r"5 1
1 2",
            "5"
        ),
        (
            r"60 3
5 8
1 3
10 15",
            "221823067"
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