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
    let mut a: Vec<isize> = Vec::new();
    let mut b: Vec<isize> = Vec::new();
    for _ in 0..n {
        let ai: isize = iterator.next().unwrap().parse().unwrap();
        let bi: isize = iterator.next().unwrap().parse().unwrap();
        a.push(ai);
        b.push(bi);
    }

    let mut c = a.clone();
    c.sort();
    let mut d = b.clone();
    d.sort();

    let enter = c[n / 2];
    let exit = d[n / 2];
    let mut ans = 0;
    for i in 0..n {
        ans += (a[i] - enter).abs();
        ans += (a[i] - b[i]).abs();
        ans += (b[i] - exit).abs();
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3
5 7
2 6
8 10",
            "18"
        ),
        (
            r"5
1 71
43 64
13 35
14 54
79 85",
            "334"
        ),
        (
            r"11
15004200 341668840
277786703 825590503
85505967 410375631
797368845 930277710
90107929 763195990
104844373 888031128
338351523 715240891
458782074 493862093
189601059 534714600
299073643 971113974
98291394 443377420",
            "8494550716"
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