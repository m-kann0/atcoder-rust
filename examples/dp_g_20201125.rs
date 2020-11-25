#![allow(non_snake_case)]

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
    let m: usize = iterator.next().unwrap().parse().unwrap();
    let mut graph: Vec<Vec<usize>> = vec![Vec::new(); n];
    for _ in 0..m {
        let xi: usize = iterator.next().unwrap().parse().unwrap();
        let yi: usize = iterator.next().unwrap().parse().unwrap();
        graph[xi - 1].push(yi - 1);
    }

    // let order = topological_sort::topological_sort(&graph);
    //
    // let mut dp = vec![0_usize; n];
    // for u in order {
    //     for &v in &graph[u] {
    //         dp[v] = max(dp[v], dp[u] + 1);
    //     }
    // }
    //
    // let mut ans = 0;
    // for i in 0..n {
    //     ans = max(ans, dp[i]);
    // }
    // ans.to_string()

    let mut result = 0;
    let mut memo: Vec<Option<usize>> = vec![None; n];
    for v in 0..n {
        result = max(result, rec(&graph, v, &mut memo));
    }
    result.to_string()
}

fn rec(graph: &Vec<Vec<usize>>, v: usize, memo: &mut Vec<Option<usize>>) -> usize {
    if let Some(d) = memo[v] {
        return d;
    }

    let mut result = 0;
    for &nv in &graph[v] {
        result = max(result, rec(graph, nv, memo) + 1);
    }
    memo[v] = Some(result);
    result
}


mod topological_sort {
    pub fn topological_sort(graph: &Vec<Vec<usize>>) -> Vec<usize> {
        let mut seen = vec![false; graph.len()];
        let mut result = Vec::new();
        for v in 0..graph.len() {
            if seen[v] {
                continue;
            }
            rec(graph, v, &mut seen, &mut result);
        }
        result.reverse();
        result
    }

    fn rec(graph: &Vec<Vec<usize>>, v: usize, seen: &mut Vec<bool>, result: &mut Vec<usize>) {
        seen[v] = true;
        for &next_v in &graph[v] {
            if seen[next_v] {
                continue;
            }
            rec(graph, next_v, seen, result);
        }
        result.push(v);
    }
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4 5
1 2
1 3
3 2
2 4
3 4",
            "3"
        ),
        (
            r"6 3
2 3
4 5
5 6",
            "2"
        ),
        (
            r"5 8
5 3
2 3
2 4
5 2
5 1
1 4
4 3
1 3",
            "3"
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