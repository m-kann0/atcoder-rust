use std::io::Read;
use std::collections::VecDeque;
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
        let ai: usize = iterator.next().unwrap().parse().unwrap();
        let bi: usize = iterator.next().unwrap().parse().unwrap();
        let ai = ai - 1;
        let bi = bi - 1;
        graph[ai].push(bi);
        graph[bi].push(ai);
    }

    let mut ans: usize = 0;
    let mut visited: Vec<bool> = vec![false; n];
    for i in 0..n {
        if visited[i] {
            continue;
        }
        let mut a: usize = 0;
        let mut q: VecDeque<usize> = VecDeque::new();
        visited[i] = true;
        q.push_back(i);
        a += 1;
        while let Some(u) = q.pop_front() {
            for &v in &graph[u] {
                if visited[v] {
                    continue;
                }
                visited[v] = true;
                q.push_back(v);
                a += 1;
            }
        }
        ans = max(ans, a);
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5 3
1 2
3 4
5 1",
            "3"
        ),
        (
            r"4 10
1 2
2 1
1 2
2 1
1 2
1 3
1 4
2 3
2 4
3 4",
            "4"
        ),
        (
            r"10 4
3 1
4 1
5 9
2 6",
            "3"
        ),
        (
            r"5 0",
            "1"
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