#![allow(non_snake_case)]

use std::io::Read;
use std::collections::VecDeque;

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
    let k: usize = iterator.next().unwrap().parse().unwrap();
    let h: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();
    let c: Vec<usize> = (0..k).map(|_| {
        let ci: usize = iterator.next().unwrap().parse().unwrap();
        ci - 1
    }).collect();
    let mut graph = vec![vec![]; n];
    for _ in 0..m {
        let ai: usize = iterator.next().unwrap().parse().unwrap();
        let bi: usize = iterator.next().unwrap().parse().unwrap();
        let ai = ai - 1;
        let bi = bi - 1;
        if h[ai] < h[bi] {
            graph[ai].push(bi);
        } else {
            graph[bi].push(ai);
        }
    }

    let mut q = VecDeque::new();
    const INF: usize = 1_000_000_000_000_000_000;
    let mut dist = vec![INF; n];
    for i in 0..k {
        q.push_back(c[i]);
        dist[c[i]] = 0;
    }
    while let Some(u) = q.pop_front() {
        for &v in &graph[u] {
            if dist[v] != INF {
                continue;
            }
            q.push_back(v);
            dist[v] = dist[u] + 1;
        }
    }

    let mut output = String::new();
    for i in 0..n {
        if dist[i] == INF {
            output.push_str(&format!("{}\n", -1));
        } else {
            output.push_str(&format!("{}\n", dist[i]));
        }
    }
    output
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5 5 2
1 2 3 4 5
1 2
1 2
1 3
4 2
4 3
3 5",
            "0
0
1
1
2"
        ),
        (
            r"5 6 2
6 5 9 15 3
4 2
1 5
2 5
2 4
1 3
4 3
2 1",
            "1
0
2
0
-1"
        ),
        (
            r"5 4 2
3 10 5 8 2
3 5
3 2
3 1
4 5
2 1",
            "-1
1
0
1
0"
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