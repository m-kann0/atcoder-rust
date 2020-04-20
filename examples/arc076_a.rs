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

    if n > m + 1 || m > n + 1 {
        return "0".to_string();
    }

    let f_n = fact(n);
    let f_m = fact(m);

    let mut ans = f_n * f_m % MOD;
    if n == m {
        ans = ans * 2 % MOD;
    }
    return ans.to_string();
}

const MOD: usize = 1_000_000_007;

fn fact(mut x: usize) -> usize {
    let mut result = 1;
    while x > 0 {
        result = result * x % MOD;
        x -= 1;
    }
    result
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2 2",
            "8"
        ),
        (
            r"3 2",
            "12"
        ),
        (
            r"1 8",
            "0"
        ),
        (
            r"100000 100000",
            "530123477"
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