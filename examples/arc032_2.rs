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
        let ai: usize = iterator.next().unwrap().parse().unwrap();
        let bi: usize = iterator.next().unwrap().parse().unwrap();
        graph[ai - 1].push(bi - 1);
        graph[bi - 1].push(ai - 1);
    }

    let mut colors: Vec<Option<usize>> = vec![None; n];
    let mut color: usize = 0;
    for i in 0..n {
        if colors[i].is_some() {
            continue;
        }

        let mut q = VecDeque::new();
        q.push_back(i);
        colors[i] = Some(color);
        while let Some(u) = q.pop_front() {
            for &v in &graph[u] {
                if colors[v].is_some() {
                    continue;
                }
                q.push_back(v);
                colors[v] = Some(color);
            }
        }

        color += 1;
    }

    (color - 1).to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4 2
1 2
1 3",
            "1"
        ),
        (
            r"6 4
1 2
2 3
1 3
5 6",
            "2"
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