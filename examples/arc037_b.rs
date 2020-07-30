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

    let mut graph: Vec<Vec<bool>> = vec![vec![false; n]; n];
    for _ in 0..m {
        let u: usize = iterator.next().unwrap().parse().unwrap();
        let v: usize = iterator.next().unwrap().parse().unwrap();
        let u = u - 1;
        let v = v - 1;
        graph[u][v] = true;
        graph[v][u] = true;
    }

    let mut ans: usize = 0;
    let mut visited: Vec<bool> = vec![false; n];
    for i in 0..n {
        if visited[i] {
            continue;
        }
        let mut is_tree = true;
        let mut q: VecDeque<_> = VecDeque::new();
        q.push_back(i);
        visited[i] = true;
        while !q.is_empty() {
            let u = q.pop_front().unwrap();
            for v in 0..n {
                if !graph[u][v] {
                    continue;
                }
                if visited[v] {
                    is_tree = false;
                    continue;
                }
                q.push_back(v);
                visited[v] = true;
                graph[u][v] = false;
                graph[v][u] = false;
            }
        }
        if is_tree {
            ans += 1;
        }
    }
    ans.to_string()
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