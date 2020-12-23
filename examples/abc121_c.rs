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

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let m: usize = iterator.next().unwrap().parse().unwrap();
    let mut stores = Vec::new();
    for _ in 0..n {
        let a: usize = iterator.next().unwrap().parse().unwrap();
        let b: usize = iterator.next().unwrap().parse().unwrap();
        stores.push((a, b));
    }

    stores.sort();

    let mut nokori: usize = m;
    let mut ans: usize = 0;
    for i in 0..n {
        if nokori < stores[i].1 {
            ans += nokori * stores[i].0;
            nokori = 0;
        } else {
            ans += stores[i].1 * stores[i].0;
            nokori -= stores[i].1;
        }
        if nokori <= 0 {
            break;
        }
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2 5
4 9
2 4",
            "12"
        ),
        (
            r"4 30
6 18
2 5
3 10
7 9",
            "130"
        ),
        (
            r"1 100000
1000000000 100000",
            "100000000000000"
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