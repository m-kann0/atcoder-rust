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

    let mut graph: Vec<Vec<usize>> = vec![Vec::new(); n];
    for _ in 0..m {
        let a: usize = iterator.next().unwrap().parse().unwrap();
        let b: usize = iterator.next().unwrap().parse().unwrap();
        graph[a - 1].push(b - 1);
        graph[b - 1].push(a - 1);
    }

    const INF: usize = std::usize::MAX;

    let mut from: Vec<usize> = vec![INF; n];
    from[0] = 0;

    let mut q: VecDeque<usize> = VecDeque::new();
    q.push_back(0);
    while !q.is_empty() {
        let u: usize = q.pop_front().unwrap();
        for &v in graph[u].iter() {
            if from[v] == INF {
                from[v] = u;
                q.push_back(v);
            }
        }
    }

    let mut result = String::from("Yes\n");
    for i in 1..n {
        result.push_str(&format!("{}\n", from[i] + 1));
    }
    return result.trim().to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4 4
1 2
2 3
3 4
4 2",
            "Yes
1
2
2"
        ),
        (
            r"6 9
3 4
6 1
2 4
5 3
4 6
1 5
6 2
4 5
5 6",
            "Yes
6
5
5
1
1"
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