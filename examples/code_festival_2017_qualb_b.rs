use std::io::Read;
use std::collections::HashMap;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: usize = iterator.next().unwrap().parse().unwrap();

    let mut problems: HashMap<usize, usize> = HashMap::new();
    for _ in 0..n {
        let d: usize = iterator.next().unwrap().parse().unwrap();
        *problems.entry(d).or_insert(0) += 1;
    }

    let m: usize = iterator.next().unwrap().parse().unwrap();

    let mut required: HashMap<usize, usize> = HashMap::new();
    for _ in 0..m {
        let t: usize = iterator.next().unwrap().parse().unwrap();
        *required.entry(t).or_insert(0) += 1;
    }

    for (key, value) in required {
        let default = 0;
        let count = problems.get(&key).unwrap_or(&default);
        if *count < value {
            return String::from("NO");
        }
    }

    return String::from("YES");
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5
3 1 4 1 5
3
5 4 3",
            "YES"
        ),
        (
            r"7
100 200 500 700 1200 1600 2000
6
100 200 500 700 1600 1600",
            "NO"
        ),
        (
            r"1
800
5
100 100 100 100 100",
            "NO"
        ),
        (
            r"15
1 2 2 3 3 3 4 4 4 4 5 5 5 5 5
9
5 4 3 2 1 2 3 4 5",
            "YES"
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