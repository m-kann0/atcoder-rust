#![allow(non_snake_case)]

use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let mut a = vec![];
    let price: usize = iterator.next().unwrap().parse().unwrap();
    a.push((price * 8, 1, price));
    let price: usize = iterator.next().unwrap().parse().unwrap();
    a.push((price * 4, 2, price));
    let price: usize = iterator.next().unwrap().parse().unwrap();
    a.push((price * 2, 4, price));
    let price: usize = iterator.next().unwrap().parse().unwrap();
    a.push((price * 1, 8, price));

    a.sort();

    let mut n: usize = iterator.next().unwrap().parse().unwrap();
    n *= 4;
    let mut ans = 0;
    for i in 0..4 {
        ans += a[i].2 * (n / a[i].1);
        n %= a[i].1;
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"20 30 70 90
3",
            "150"
        ),
        (
            r"10000 1000 100 10
1",
            "100"
        ),
        (
            r"10 100 1000 10000
1",
            "40"
        ),
        (
            r"12345678 87654321 12345678 87654321
123456789",
            "1524157763907942"
        ),
    ];

    let mut all_ok = true;
    for (i, case) in cases.iter().enumerate() {
        print!("case {} : ", i);

        let expected = case.1;
        let actual = solve(case.0);

        if expected.trim() == actual.trim() {
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