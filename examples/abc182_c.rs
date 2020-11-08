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

    let mut n: usize = iterator.next().unwrap().parse().unwrap();

    let mut sum = 0;
    let mut m: Vec<usize> = vec![0; 3];
    let mut k: usize = 0;
    while n > 0 {
        let i = n % 10;
        sum += i;
        m[i % 3] += 1;
        k += 1;
        n /= 10;
    }

    const INF: usize = 10000;
    let mut ans = INF;
    for i1 in 0..=2 {
        for i2 in 0..=2 {
            if i1 > m[1] {
                continue;
            }
            if i2 > m[2] {
                continue;
            }
            if i1 + i2 >= k {
                continue;
            }
            let mut now = sum;
            now -= i1 * 1;
            now -= i2 * 2;
            if now % 3 == 0 {
                ans = min(ans, i1 + i2);
            }
        }
    }
    if ans == INF {
        return "-1".to_string();
    }
    ans.to_string()
}

#[test]
fn test2() {
    let mut n = 3;
    for n in 1..=100 {
        let mut has_zero = false;
        let mut nn = n;
        while nn > 0 {
            let mut amari = nn % 10;
            if amari == 0 {
                has_zero = true;
                break;
            }
            nn /= 10;
        }
        if has_zero {
            continue;
        }
        println!("{}: {}", n, solve(&n.to_string()));
    }
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"35",
            "1"
        ),
        (
            r"369",
            "0"
        ),
        (
            r"6227384",
            "1"
        ),
        (
            r"4622738",
            "1"
        ),
        (
            r"11",
            "-1"
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
