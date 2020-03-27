use std::io::Read;
use std::cmp::min;

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

    let mut x: Vec<isize> = (0..m)
        .map(|_| iterator.next().unwrap().parse().unwrap())
        .collect();
    x.sort();

    let mut diffs: Vec<usize> = Vec::with_capacity(m);
    for i in 0..(m - 1) {
        diffs.push((x[i + 1] - x[i]) as usize);
    }

    diffs.sort();
    for _ in 0..min(n - 1, m - 1) {
        diffs.pop();
    }

    let ans: usize = diffs.iter().sum();

    return ans.to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2 5
10 12 1 2 14",
            "5"
        ),
        (
            r"3 7
-10 -3 0 9 -100 2 17",
            "19"
        ),
        (
            r"100 1
-100000",
            "0"
        ),
        (
            r"1 3
1 2 3",
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