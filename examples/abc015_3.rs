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
    let k: usize = iterator.next().unwrap().parse().unwrap();

    let mut t: Vec<Vec<usize>> = vec![Vec::new(); n];
    for i in 0..n {
        for _ in 0..k {
            t[i].push(iterator.next().unwrap().parse().unwrap());
        }
    }

    let result = dfs(0, 0, n, &t);
    return if result {
        "Found".to_string()
    } else {
        "Nothing".to_string()
    };
}

fn dfs(depth: usize, current: usize, n: usize, t: &Vec<Vec<usize>>) -> bool {
    if depth == n {
        return current == 0;
    }

    let mut result = false;
    for &x in &t[depth] {
        let r = dfs(depth + 1, current ^ x, n, t);
        if r {
            result = true;
        }
    }
    return result;
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 4
1 3 5 17
2 4 2 3
1 3 2 9",
            "Found"
        ),
        (
            r"5 3
89 62 15
44 36 17
4 24 24
25 98 99
66 33 57",
            "Nothing"
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