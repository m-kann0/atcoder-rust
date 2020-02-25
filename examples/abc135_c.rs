use std::io::Read;
use std::cmp::max;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: usize = iterator.next().unwrap().parse().unwrap();

    let mut a: Vec<i64> = Vec::with_capacity(n + 1);
    let mut before: i64 = 0;
    for _i in 0..(n + 1) {
        let ai = iterator.next().unwrap().parse().unwrap();
        a.push(ai);
        before += ai;
    }

    let mut after: i64 = 0;
    for i in 0..n {
        let mut b: i64 = iterator.next().unwrap().parse().unwrap();
        if a[i] < b {
            b -= a[i];
            a[i] = 0;
            a[i + 1] = max(0, a[i + 1] - b);
        } else {
            a[i] -= b;
        }
        after += a[i];
    }
    after += a[n];

    let ans: i64 = before - after;

    return ans.to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2
3 5 2
4 5",
            "9"
        ),
        (
            r"3
5 6 3 8
5 100 8",
            "22"
        ),
        (
            r"2
100 1 1
1 100",
            "3"
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