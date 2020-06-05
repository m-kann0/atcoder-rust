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
    let mut a: Vec<usize> = Vec::new();
    for _ in 0..n {
        a.push(iterator.next().unwrap().parse().unwrap());
    }

    let mut l: Vec<usize> = vec![0; n + 2];
    for i in 1..(n + 1) {
        l[i] = gcd(l[i - 1], a[i - 1]);
    }

    let mut r: Vec<usize> = vec![0; n + 2];
    for i in (1..(n + 1)).rev() {
        r[i] = gcd(r[i + 1], a[i - 1]);
    }

    let mut ans: usize = 0;
    for i in 1..(n + 1) {
        ans = max(ans, gcd(l[i - 1], r[i + 1]));
    }
    return ans.to_string();
}

fn gcd(mut x: usize, mut y: usize) -> usize {
    while y > 0 {
        let r = x % y;
        x = y;
        y = r;
    }
    return x;
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3
7 6 8",
            "2"
        ),
        (
            r"3
12 15 18",
            "6"
        ),
        (
            r"2
1000000000 1000000000",
            "1000000000"
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