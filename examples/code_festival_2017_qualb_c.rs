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
    let mut graph = vec![vec![]; n];
    for _ in 0..m {
        let a: usize = iterator.next().unwrap().parse().unwrap();
        let b: usize = iterator.next().unwrap().parse().unwrap();
        graph[a - 1].push(b - 1);
        graph[b - 1].push(a - 1);
    }

    let mut is_two_part = true;

    let mut color = vec![None; n];
    let mut q = VecDeque::new();
    color[0] = Some(0);
    q.push_back(0);
    while let Some(u) = q.pop_front() {
        let color_u = color[u].unwrap();
        for &v in &graph[u] {
            if let Some(color_v) = color[v] {
                if color_u == color_v {
                    is_two_part = false;
                    break;
                } else {
                    continue;
                }
            }
            color[v] = Some((color_u + 1) % 2);
            q.push_back(v);
        }
        if !is_two_part {
            break;
        }
    }

    if is_two_part {
        let mut black = 0_usize;
        let mut white = 0_usize;
        for i in 0..n {
            if color[i].unwrap() == 0 {
                black += 1;
            } else {
                white += 1;
            }
        }
        let ans = black * white - m;
        return ans.to_string();
    }

    let ans = n * (n - 1) / 2 - m;
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"6 5
1 2
2 3
3 4
4 5
5 6",
            "4"
        ),
        (
            r"5 5
1 2
2 3
3 1
5 4
5 1",
            "5"
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