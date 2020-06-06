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
    let q: usize = iterator.next().unwrap().parse().unwrap();

    let mut graph: Vec<Vec<usize>> = vec![Vec::new(); n];
    for _ in 0..m {
        let u: usize = iterator.next().unwrap().parse().unwrap();
        let v: usize = iterator.next().unwrap().parse().unwrap();
        graph[u - 1].push(v - 1);
        graph[v - 1].push(u - 1);
    }

    let mut color: Vec<usize> = vec![0; n];
    for i in 0..n {
        let ci: usize = iterator.next().unwrap().parse().unwrap();
        color[i] = ci;
    }

    let mut result: String = String::new();
    for _ in 0..q {
        let query: usize = iterator.next().unwrap().parse().unwrap();
        if query == 1 {
            let x: usize = iterator.next().unwrap().parse().unwrap();
            result.push_str(&format!("{}\n", color[x - 1]));
            for v in &graph[x - 1] {
                color[*v] = color[x - 1];
            }
        } else {
            let x: usize = iterator.next().unwrap().parse().unwrap();
            let y: usize = iterator.next().unwrap().parse().unwrap();
            result.push_str(&format!("{}\n", color[x - 1]));
            color[x - 1] = y;
        }
    }

    return result.trim().to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 2 3
1 2
2 3
5 10 15
1 2
2 1 20
1 1",
            "10
10
20"
        ),
        (
            r"30 10 20
11 13
30 14
6 4
7 23
30 8
17 4
6 23
24 18
26 25
9 3
18 4 36 46 28 16 34 19 37 23 25 7 24 16 17 41 24 38 6 29 10 33 38 25 47 8 13 8 42 40
2 1 9
1 8
1 9
2 20 24
2 26 18
1 20
1 26
2 24 31
1 4
2 21 27
1 25
1 29
2 10 14
2 2 19
2 15 36
2 28 6
2 13 5
1 12
1 11
2 14 43",
            "18
19
37
29
8
24
18
25
46
10
18
42
23
4
17
8
24
7
25
16"
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