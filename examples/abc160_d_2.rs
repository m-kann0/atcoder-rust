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
    let x: usize = iterator.next().unwrap().parse().unwrap();
    let y: usize = iterator.next().unwrap().parse().unwrap();
    let mut graph: Vec<Vec<usize>> = vec![Vec::new(); n];
    for i in 0..n {
        if i != 0 {
            graph[i].push(i - 1);
        }
        if i != n - 1 {
            graph[i].push(i + 1);
        }
    }
    graph[x - 1].push(y - 1);
    graph[y - 1].push(x - 1);

    const INF: usize = std::usize::MAX;

    let mut ans: Vec<usize> = vec![0; n + 5];
    for i in 0..n {
        let mut d: Vec<usize> = vec![INF; n];
        let mut q: VecDeque<usize> = VecDeque::new();
        d[i] = 0;
        q.push_back(i);
        while let Some(u) = q.pop_front() {
            for &v in &graph[u] {
                if d[v] != INF {
                    continue;
                }
                d[v] = d[u] + 1;
                q.push_back(v);
            }
        }
        for j in 0..n {
            if i < j {
                ans[d[j]] += 1;
            }
        }
    }

    let mut result = String::new();
    for i in 1..=(n - 1) {
        result.push_str(&format!("{}\n", ans[i]));
    }
    result.trim().to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5 2 4",
            "5
4
1
0"
        ),
        (
            r"3 1 3",
            "3
0"
        ),
        (
            r"7 3 7",
            "7
8
4
2
0
0"
        ),
        (
            r"10 4 8",
            "10
12
10
8
4
1
0
0
0"
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