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
    let q: usize = iterator.next().unwrap().parse().unwrap();
    let mut graph: Vec<Vec<usize>> = vec![Vec::new(); n];
    for _ in 0..(n - 1) {
        let a: usize = iterator.next().unwrap().parse().unwrap();
        let b: usize = iterator.next().unwrap().parse().unwrap();
        graph[a - 1].push(b - 1);
        graph[b - 1].push(a - 1);
    }
    let mut x: Vec<usize> = vec![0; n];
    for _ in 0..q {
        let pi: usize = iterator.next().unwrap().parse().unwrap();
        let xi: usize = iterator.next().unwrap().parse().unwrap();
        x[pi - 1] += xi;
    }

    let mut point = vec![None; n];
    let mut q = VecDeque::new();
    point[0] = Some(x[0]);
    q.push_back(0);
    while let Some(u) = q.pop_front() {
        for &v in &graph[u] {
            if point[v] != None {
                continue;
            }
            point[v] = Some(point[u].unwrap() + x[v]);
            q.push_back(v);
        }
    }

    let mut result = String::new();
    for i in 0..n {
        result.push_str(&format!("{} ", point[i].unwrap()));
    }
    result
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4 3
1 2
2 3
2 4
2 10
1 100
3 1",
            "100 110 111 110"
        ),
        (
            r"6 2
1 2
1 3
2 4
3 6
2 5
1 10
1 10",
            "20 20 20 20 20 20"
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