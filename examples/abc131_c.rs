use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let a: u64 = iterator.next().unwrap().parse().unwrap();
    let b: u64 = iterator.next().unwrap().parse().unwrap();
    let c: u64 = iterator.next().unwrap().parse().unwrap();
    let d: u64 = iterator.next().unwrap().parse().unwrap();

    // 遅い
    // let lcm = kobaisu(c, d, b);

    let lcm = lcm(c, d);

    let x = b / c + b / d - b / lcm;
    let y = (a - 1) / c + (a - 1) / d - (a - 1) / lcm;

    let ans = (b - x) - (a - 1 - y);

    return ans.to_string();
}

// 遅い
fn kobaisu(s: u64, t: u64, max: u64) -> u64 {
    let mut u: u64 = s;
    let mut v: u64 = t;
    while u != v {
        if u > max || v > max {
            return std::u64::MAX
        }
        if u < v {
            u += s;
        } else {
            v += t;
        }
    }
    u
}

/// 最大公約数（greatest common divisor）を求める
fn gcd(m: u64, n: u64) -> u64 {
    if n > m {
        gcd(n, m)
    } else if n == 0 {
        m
    } else {
        gcd(n, m % n)
    }
}

/// 最小公倍数（least common multiple）を求める
fn lcm(m: u64, n: u64) -> u64 {
    m / gcd(m, n) * n
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4 9 2 3",
            "2"
        ),
        (
            r"10 40 6 8",
            "23"
        ),
        (
            r"314159265358979323 846264338327950288 419716939 937510582",
            "532105071133627368"
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