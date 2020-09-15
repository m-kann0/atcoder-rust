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
    let mut graph: Vec<Vec<(usize, usize)>> = vec![Vec::new(); n];
    for _ in 0..(n - 1) {
        let u: usize = iterator.next().unwrap().parse().unwrap();
        let v: usize = iterator.next().unwrap().parse().unwrap();
        let w: usize = iterator.next().unwrap().parse().unwrap();
        graph[u - 1].push((v - 1, w));
        graph[v - 1].push((u - 1, w));
    }

    let mut colors: Vec<usize> = vec![2; n];
    let mut q: VecDeque<usize> = VecDeque::new();
    colors[0] = 0;
    q.push_back(0);
    while let Some(u) = q.pop_front() {
        for &(v, w) in &graph[u] {
            if colors[v] != 2 {
                continue;
            }
            if w % 2 == 0 {
                colors[v] = colors[u];
            } else {
                colors[v] = if colors[u] == 0 { 1 } else { 0 };
            }
            q.push_back(v);
        }
    }
    let mut result = String::new();
    for i in 0..n {
        result.push_str(&format!("{}\n", colors[i]));
    }
    result.trim().to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3
1 2 2
2 3 1",
            "0
0
1"
        ),
        (
            r"5
2 5 2
2 3 10
1 3 8
3 4 2",
            "1
0
1
0
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