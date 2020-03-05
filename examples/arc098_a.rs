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

    let mut s: Vec<char> = Vec::with_capacity(n);
    for c in iterator.next().unwrap().chars() {
        s.push(c);
    }

    let mut ans = 0;
    for i in 1..n {
        if s[i] == 'E' {
            ans += 1;
        }
    }

    let mut prev = ans;
    for i in 1..n {
        let mut a = prev;
        if s[i] == 'E' {
            a -= 1;
        }
        if s[i - 1] == 'W' {
            a += 1;
        }
        ans = min(ans, a);
        prev = a;
    }

    return ans.to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5
WEEWW",
            "1"
        ),
        (
            r"12
WEWEWEEEWWWE",
            "4"
        ),
        (
            r"8
WWWWWEEE",
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