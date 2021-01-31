#![allow(non_snake_case)]

use std::io::Read;
use std::collections::VecDeque;
use std::cmp::min;

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
    let mut graph = vec![vec![]; n];
    for _ in 0..m {
        let ai: usize = iterator.next().unwrap().parse().unwrap();
        let bi: usize = iterator.next().unwrap().parse().unwrap();
        let ai = ai - 1;
        let bi = bi - 1;
        graph[ai].push(bi);
        graph[bi].push(ai);
    }
    let k: usize = iterator.next().unwrap().parse().unwrap();
    let mut c = vec![];
    for _ in 0..k {
        let ci: usize = iterator.next().unwrap().parse().unwrap();
        let ci = ci - 1;
        c.push(ci);
    }

    const INF: usize = 1_000_000_000;

    let mut dist = vec![vec![0; k]; k];
    for i in 0..k {
        let mut d = vec![INF; n];
        d[c[i]] = 0;
        let mut q = VecDeque::new();
        q.push_back(c[i]);
        while !q.is_empty() {
            let u = q.pop_front().unwrap();
            for &v in &graph[u] {
                if d[v] != INF {
                    continue;
                }
                d[v] = d[u] + 1;
                q.push_back(v);
            }
        }
        for j in 0..k {
            dist[i][j] = d[c[j]];
        }
    }

    // eprintln!("dist = {:?}", dist);

    let mut dp = vec![vec![INF; k]; 1 << k];
    for i in 0..k {
        dp[1 << i][i] = 1;
    }
    for bit in 0..(1 << k) {
        for u in 0..k {
            for v in 0..k {
                if bit & (1 << u) > 0 && bit & (1 << v) == 0 {
                    dp[bit | (1 << v)][v] = min(dp[bit | (1 << v)][v], dp[bit][u] + dist[u][v]);
                }
            }
        }
    }
    let mut ans = INF;
    for i in 0..k {
        ans = min(ans, dp[(1 << k) - 1][i]);
    }
    if ans >= INF {
        return "-1".to_string();
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4 3
1 4
2 4
3 4
3
1 2 3",
            "5"
        ),
        (
            r"4 3
1 4
2 4
1 2
3
1 2 3",
            "-1"
        ),
        (
            r"10 10
3 9
3 8
8 10
2 10
5 8
6 8
5 7
6 7
1 6
2 4
4
1 2 7 9",
            "11"
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