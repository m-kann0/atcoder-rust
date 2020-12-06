#![allow(non_snake_case)]

use std::io::Read;
use std::collections::BinaryHeap;
use std::cmp::{Reverse, min};

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
    let s: usize = iterator.next().unwrap().parse().unwrap();
    let t: usize = iterator.next().unwrap().parse().unwrap();
    let s = s - 1;
    let t = t - 1;
    let mut graph1 = vec![vec![]; n];
    let mut graph2 = vec![vec![]; n];
    for _ in 0..m {
        let u: usize = iterator.next().unwrap().parse().unwrap();
        let v: usize = iterator.next().unwrap().parse().unwrap();
        let u = u - 1;
        let v = v - 1;
        let a: usize = iterator.next().unwrap().parse().unwrap();
        let b: usize = iterator.next().unwrap().parse().unwrap();
        graph1[u].push(Edge{to: v, cost: a});
        graph1[v].push(Edge{to: u, cost: a});
        graph2[u].push(Edge{to: v, cost: b});
        graph2[v].push(Edge{to: u, cost: b});
    }

    const INF: usize = std::usize::MAX;
    let mut dist1 = vec![INF; n];
    let mut pq = BinaryHeap::new();
    dist1[s] = 0;
    pq.push(Reverse((0, s)));
    while let Some(Reverse((d, u))) = pq.pop() {
        if d > dist1[u] {
            continue;
        }
        for &e in &graph1[u] {
            if dist1[e.to] > dist1[u] + e.cost {
                dist1[e.to] = dist1[u] + e.cost;
                pq.push(Reverse((dist1[e.to], e.to)));
            }
        }
    }
    let mut dist2 = vec![INF; n];
    let mut pq = BinaryHeap::new();
    dist2[t] = 0;
    pq.push(Reverse((0, t)));
    while let Some(Reverse((d, u))) = pq.pop() {
        if d > dist2[u] {
            continue;
        }
        for &e in &graph2[u] {
            if dist2[e.to] > dist2[u] + e.cost {
                dist2[e.to] = dist2[u] + e.cost;
                pq.push(Reverse((dist2[e.to], e.to)));
            }
        }
    }

    let mut result = vec![INF; n];
    result[n - 1] = dist1[n - 1] + dist2[n - 1];
    for i in (0..=(n - 2)).rev() {
        result[i] = min(result[i + 1], dist1[i] + dist2[i]);
    }

    let mut ans = String::new();
    for i in 0..n {
        ans.push_str(&format!("{}\n", 1_000_000_000_000_000_usize - result[i]));
    }
    ans
}

#[derive(Copy, Clone)]
struct Edge {
    to: usize,
    cost: usize,
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4 3 2 3
1 4 1 100
1 2 1 10
1 3 20 1",
            "999999999999998
999999999999989
999999999999979
999999999999897"
        ),
        (
            r"8 12 3 8
2 8 685087149 857180777
6 7 298270585 209942236
2 4 346080035 234079976
2 5 131857300 22507157
4 8 30723332 173476334
2 6 480845267 448565596
1 4 181424400 548830121
4 5 57429995 195056405
7 8 160277628 479932440
1 6 475692952 203530153
3 5 336869679 160714712
2 7 389775999 199123879",
            "999999574976994
999999574976994
999999574976994
999999574976994
999999574976994
999999574976994
999999574976994
999999574976994"
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