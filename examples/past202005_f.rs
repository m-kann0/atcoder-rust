use std::io::Read;
use std::collections::HashSet;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let mut a: Vec<HashSet<char>> = vec![HashSet::new(); n];
    for i in 0..n {
        let line = iterator.next().unwrap();
        for c in line.chars() {
            a[i].insert(c);
        }
    }

    let mut head = String::new();
    let mut tail = String::new();
    for i in 0..(n / 2) {
        let inter = a[i].intersection(&a[n - 1 - i]);
        if let Some(c) = inter.last() {
            head.push(*c);
            tail.push(*c);
        } else {
            return "-1".to_string();
        }
    }
    if n % 2 == 1 {
        let c = a[n / 2].iter().last().unwrap();
        head.push(*c);
    }
    return head + &tail.chars().rev().collect::<String>();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2
yc
ys",
            "yy"
        ),
        (
            r"2
rv
jh",
            "-1"
        ),
        (
            r"3
yca
yca
ysa",
            "yyy"
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