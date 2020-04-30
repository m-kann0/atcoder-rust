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

    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];

    for _ in 0..m {
        let a: usize = iterator.next().unwrap().parse().unwrap();
        let b: usize = iterator.next().unwrap().parse().unwrap();
        graph[a - 1].push(b - 1);
        graph[b - 1].push(a - 1);
    }

    let mut result = String::new();
    for u in 0..n {
        let mut visited: Vec<bool> = vec![false; n];
        visited[u] = true;

        for v1 in graph[u].iter() {
            visited[*v1] = true;
        }

        let mut count = 0;
        for v1 in graph[u].iter() {
            for v2 in graph[*v1].iter() {
                if !visited[*v2] {
                    count += 1;
                    visited[*v2] = true;
                }
            }
        }
        result.push_str(&format!("{}\n", count));
    }

    return result.trim().to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 2
1 2
2 3",
            "1
0
1"
        ),
        (
            r"3 3
1 2
1 3
2 3",
            "0
0
0"
        ),
        (
            r"8 12
1 6
1 7
1 8
2 5
2 6
3 5
3 6
4 5
4 8
5 6
5 7
7 8",
            "4
4
4
5
2
3
4
2"
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