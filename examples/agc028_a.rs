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
    let s: Vec<char> = iterator.next().unwrap().chars().collect();
    let t: Vec<char> = iterator.next().unwrap().chars().collect();

    let g = gcd(n, m);
    for i in 0..g {
        if s[n / g * i] != t[m / g * i] {
            return "-1".to_string();
        }
    }

    return (n * m / g).to_string();
}

fn gcd(mut m: usize, mut n: usize) -> usize {
    while n != 0 {
        let r = m % n;
        m = n;
        n = r;
    }
    m
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 2
acp
ae",
            "6"
        ),
        (
            r"6 3
abcdef
abc",
            "-1"
        ),
        (
            r"15 9
dnsusrayukuaiia
dujrunuma",
            "45"
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