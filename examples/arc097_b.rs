use std::io::Read;
use std::collections::{VecDeque, HashSet};

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
    let p: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse::<usize>().unwrap() - 1).collect();
    let mut graph: Vec<Vec<usize>> = vec![Vec::new(); n];
    for _ in 0..m {
        let xi: usize = iterator.next().unwrap().parse().unwrap();
        let yi: usize = iterator.next().unwrap().parse().unwrap();
        graph[xi - 1].push(yi - 1);
        graph[yi - 1].push(xi - 1);
    }

    let mut ans: usize = 0;
    let mut visited: Vec<bool> = vec![false; n];
    for i in 0..n {
        if visited[i] {
            continue;
        }
        let mut q: VecDeque<usize> = VecDeque::new();
        q.push_back(i);
        visited[i] = true;
        let mut nodes = HashSet::new();
        let mut numbers = HashSet::new();
        nodes.insert(i);
        numbers.insert(p[i]);
        if i == p[i] {
            ans += 1;
        }
        while let Some(u) = q.pop_front() {
            for &node in &graph[u] {
                if visited[node] {
                    continue;
                }
                q.push_back(node);
                visited[node] = true;
                let number = p[node];
                if node == number {
                    ans += 1;
                }
                if numbers.contains(&node) {
                    ans += 1;
                }
                if nodes.contains(&number) {
                    ans += 1;
                }
                numbers.insert(number);
                nodes.insert(node);
            }
        }
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5 2
5 3 1 4 2
1 3
5 4",
            "2"
        ),
        (
            r"3 2
3 2 1
1 2
2 3",
            "3"
        ),
        (
            r"10 8
5 3 6 8 7 10 9 1 2 4
3 1
4 1
5 9
2 5
6 5
3 5
8 9
7 9",
            "8"
        ),
        (
            r"5 1
1 2 3 4 5
1 5",
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