use std::io::Read;

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
        let u: usize = iterator.next().unwrap().parse().unwrap();
        let v: usize = iterator.next().unwrap().parse().unwrap();
        graph[u - 1].push(v - 1);
        graph[v - 1].push(u - 1);
    }

    const INF: usize = 101;

    let mut ans: usize = 0;
    let mut visited: Vec<bool> = vec![false; n];
    for i in 0..n {
        if visited[i] {
            continue;
        }
        if dfs(&graph, &mut visited, i, INF) {
            ans += 1;
        }
    }
    ans.to_string()
}

fn dfs(graph: &Vec<Vec<usize>>, visited: &mut Vec<bool>, current: usize, from: usize) -> bool {
    if visited[current] {
        return false;
    }
    visited[current] = true;

    let mut is_tree = true;
    for &v in &graph[current] {
        if v == from {
            continue;
        }
        is_tree &= dfs(graph, visited, v, current);
    }
    is_tree
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"8 7
1 2
2 3
2 4
5 6
6 7
6 8
7 8",
            "1"
        ),
        (
            r"5 1
1 2",
            "4"
        ),
        (
            r"11 11
1 2
1 3
2 4
3 5
4 6
5 7
6 8
7 9
8 10
9 11
10 11",
            "0"
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