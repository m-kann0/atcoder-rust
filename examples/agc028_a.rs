use std::io::Read;
use std::collections::HashMap;

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

    let lcm = lcm(n, m);
    let mut x: HashMap<usize, char> = HashMap::new();
    for i in 0..n {
        x.insert(lcm / n * i, s[i]);
    }
    for j in 0..m {
        if x.contains_key(&(lcm / m * j)) && *x.get(&(lcm / m * j)).unwrap() != t[j] {
            return "-1".to_string();
        }
    }

    return lcm.to_string();
}

fn lcm(m: usize, n: usize) -> usize {
    return m / gcd(m, n) * n;
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