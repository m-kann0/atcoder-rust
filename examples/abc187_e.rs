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
    let mut graph = vec![vec![]; n];
    let mut e = vec![];
    for _ in 0..(n - 1) {
        let ai: usize = iterator.next().unwrap().parse().unwrap();
        let bi: usize = iterator.next().unwrap().parse().unwrap();
        let ai = ai - 1;
        let bi = bi - 1;
        graph[ai].push(bi);
        graph[bi].push(ai);
        e.push((ai, bi));
    }

    const INF: usize = 1000_000;
    let mut parent = vec![INF; n];
    let mut q = VecDeque::new();
    q.push_back(0);
    while let Some(u) = q.pop_front() {
        for &v in graph[u].iter() {
            if v != parent[u] {
                parent[v] = u;
                q.push_back(v);
            }
        }
    }

    // eprintln!("parent = {:?}", parent);

    let q: usize = iterator.next().unwrap().parse().unwrap();
    let mut c = vec![0_isize; n];
    for _ in 0..q {
        let t: usize = iterator.next().unwrap().parse().unwrap();
        let ei: usize = iterator.next().unwrap().parse().unwrap();
        let xi: isize = iterator.next().unwrap().parse().unwrap();
        let ei = ei - 1;
        let (a, b) = if t == 1 {
            (e[ei].0, e[ei].1)
        } else {
            (e[ei].1, e[ei].0)
        };

        if b != parent[a] {
            c[0] += xi;
            c[b] -= xi;
        } else {
            c[a] += xi;
        }
    }

    // eprintln!("c = {:?}", c);

    let mut q = VecDeque::new();
    q.push_back(0);
    while let Some(u) = q.pop_front() {
        for &v in graph[u].iter() {
            if v != parent[u] {
                c[v] += c[u];
                q.push_back(v);
            }
        }
    }

    let mut result = String::new();
    for i in 0..n {
        result.push_str(&format!("{}\n", c[i]));
    }
    result
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5
1 2
2 3
2 4
4 5
4
1 1 1
1 4 10
2 1 100
2 2 1000",
            "11
110
1110
110
100"
        ),
        (
            r"7
2 1
2 3
4 2
4 5
6 1
3 7
7
2 2 1
1 3 2
2 2 4
1 6 8
1 3 16
2 4 32
2 1 64",
            "72
8
13
26
58
72
5"
        ),
        (
            r"11
2 1
1 3
3 4
5 2
1 6
1 7
5 8
3 9
3 10
11 4
10
2 6 688
1 10 856
1 8 680
1 8 182
2 2 452
2 4 183
2 6 518
1 3 612
2 6 339
2 3 206",
            "1657
1657
2109
1703
1474
1657
3202
1474
1247
2109
2559"
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