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
    let mut counts: HashMap<usize, usize> = HashMap::new();
    for _ in 0..(n - 1) {
        let a: usize = iterator.next().unwrap().parse().unwrap();
        *counts.entry(a).or_insert(0) += 1;
    }

    let mut result = String::new();
    for i in 1..=n {
        let count = counts.get(&i).unwrap_or(&0);
        result.push_str(&format!("{}\n", count));
    }

    return result.trim().to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5
1 1 2 2",
            "2
2
0
0
0"
        ),
        (
            r"10
1 1 1 1 1 1 1 1 1",
            "9
0
0
0
0
0
0
0
0
0"
        ),
        (
            r"7
1 2 3 4 5 6",
            "1
1
1
1
1
1
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