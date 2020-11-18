#![allow(non_snake_case)]

use std::io::Read;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

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
    let x: usize = iterator.next().unwrap().parse().unwrap();
    let h: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();
    let mut graph = vec![Vec::new(); n];
    for _ in 0..m {
        let ai: usize = iterator.next().unwrap().parse().unwrap();
        let bi: usize = iterator.next().unwrap().parse().unwrap();
        let ti: usize = iterator.next().unwrap().parse().unwrap();
        graph[ai - 1].push((bi - 1, ti));
        graph[bi - 1].push((ai - 1, ti));
    }

    const INF: usize = 1_000_000_000_000_000;
    let mut d = vec![INF; n];
    d[0] = 0;
    let mut que: BinaryHeap<Reverse<(usize, usize)>> = BinaryHeap::new();
    que.push(Reverse((0, 0)));
    while let Some(Reverse(p)) = que.pop() {
        let from = p.1;
        if d[from] < p.0 {
            continue;
        }
        let height = x.checked_sub(d[from]).unwrap_or(0);
        for &(to, cost) in &graph[from] {
            if h[from] < cost {
                continue;
            }
            let tt = if height > cost + h[to] {
                height - cost - h[to]
            } else if cost > height {
                cost - height
            } else {
                0
            };
            if d[to] > d[from] + cost + tt {
                d[to] = d[from] + cost + tt;
                que.push(Reverse((d[to], to)));
            }
        }
    }
    if d[n - 1] >= INF {
        return "-1".to_string();
    }
    let ans = if x > d[n - 1] {
        d[n - 1] + h[n - 1] - (x - d[n - 1])
    } else {
        d[n - 1] + h[n - 1]
    };
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5 5 0
50
100
25
30
10
1 2 10
2 5 50
2 4 20
4 3 1
5 4 20",
            "110"
        ),
        (
            r"2 1 0
1
1
1 2 100",
            "-1"
        ),
        (
            r"4 3 30
50
10
20
50
1 2 10
2 3 10
3 4 10",
            "100"
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