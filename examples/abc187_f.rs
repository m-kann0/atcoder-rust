#![allow(non_snake_case)]

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

    let N: usize = iterator.next().unwrap().parse().unwrap();
    let M: usize = iterator.next().unwrap().parse().unwrap();
    let mut E = vec![0; N];
    for _ in 0..M {
        let ai: usize = iterator.next().unwrap().parse().unwrap();
        let bi: usize = iterator.next().unwrap().parse().unwrap();
        let ai = ai - 1;
        let bi = bi - 1;
        E[ai] |= 1 << bi;
        E[bi] |= 1 << ai;
    }

    let mut dp = vec![99999; 1<<N];
    dp[0] = 0;

    // 集合iがクリークになっているか調べる
    for i in 1..(1<<N) {
        for j in 0..N {
            // bitのjビット目が立っているか調べる
            if i & (1 << j) == 0 {
                continue;
            }
            // target := iのjビット目以外
            let target = i - (1 << j);
            // 頂点jからtarget全てに対して辺があるなら、iはクリーク
            if dp[target] <= 1 && E[j] & target == target {
                dp[i] = 1;
            }
            break;
        }
    }

    // 集合iが何個のクリークで実現できるかを考える
    for i in 0..(1 << N) {
        // iの部分集合を探索
        let mut j = i;
        while j > 0 {
            let k = i - j;
            dp[i] = min(dp[i], dp[j] + dp[k]);
            j = (j - 1) & i;
        }
    }
    dp[(1<<N) - 1].to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 2
1 2
1 3",
            "2"
        ),
        (
            r"4 6
1 2
1 3
1 4
2 3
2 4
3 4",
            "1"
        ),
        (
            r"10 11
9 10
2 10
8 9
3 4
5 8
1 8
5 6
2 5
3 6
6 9
1 9",
            "5"
        ),
        (
            r"18 0",
            "18"
        ),
    ];

    let mut all_ok = true;
    for (i, case) in cases.iter().enumerate() {
        print!("case {} : ", i);

        let expected = case.1;
        let actual = solve(case.0);

        if expected.trim() == actual.trim() {
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