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

    let n: isize = iterator.next().unwrap().parse().unwrap();
    let m: isize = iterator.next().unwrap().parse().unwrap();
    let l: isize = iterator.next().unwrap().parse().unwrap();

    let p: isize = iterator.next().unwrap().parse().unwrap();
    let q: isize = iterator.next().unwrap().parse().unwrap();
    let r: isize = iterator.next().unwrap().parse().unwrap();

    let mut ans = 0;
    ans = max(ans, count(n, m, l, p, q, r));
    ans = max(ans, count(n, m, l, p, r, q));
    ans = max(ans, count(n, m, l, q, p, r));
    ans = max(ans, count(n, m, l, q, r, p));
    ans = max(ans, count(n, m, l, r, p, q));
    ans = max(ans, count(n, m, l, r, q, p));

    return ans.to_string();
}

fn count(n: isize, m: isize, l: isize, x: isize, y: isize, z: isize) -> isize {
    (n / x) * (m / y) * (l / z)
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"10 10 10
1 1 1",
            r"1000"
        ),
        (
            r"10 3 1
2 1 1",
            r"15"
        ),
        (
            r"5 10 3
2 5 3",
            r"5"
        ),
        (
            r"8 8 8
1 1 9",
            r"0"
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