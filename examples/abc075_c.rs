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
    let mut edges: Vec<(usize, usize)> = Vec::new();
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    for _ in 0..m {
        let a: usize = iterator.next().unwrap().parse().unwrap();
        let b: usize = iterator.next().unwrap().parse().unwrap();
        edges.push((a - 1, b - 1));
        graph[a - 1].push(b - 1);
        graph[b - 1].push(a - 1);
    }

    let mut ans: usize = 0;
    for edge in edges {
        let mut visited: Vec<bool> = vec![false; n];
        let mut q: VecDeque<usize> = VecDeque::new();
        q.push_back(0);
        visited[0] = true;
        while !q.is_empty() {
            let u: usize = q.pop_front().unwrap();
            for &v in &graph[u] {
                if u == edge.0 && v == edge.1 || u == edge.1 && v == edge.0 {
                    continue;
                }
                if !visited[v] {
                    visited[v] = true;
                    q.push_back(v);
                }
            }
        }
        if visited.iter().any(|it| *it == false) {
            ans += 1;
        }
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"7 7
1 3
2 7
3 4
4 5
4 6
5 6
6 7",
            "4"
        ),
        (
            r"3 3
1 2
1 3
2 3",
            "0"
        ),
        (
            r"6 5
1 2
2 3
3 4
4 5
5 6",
            "5"
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