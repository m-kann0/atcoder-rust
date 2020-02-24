use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: i64 = iterator.next().unwrap().parse().unwrap();
    let a: i64 = iterator.next().unwrap().parse().unwrap();
    let b: i64 = iterator.next().unwrap().parse().unwrap();

    let mut ans: i64 = pow(2, n);
    ans = (ans - 1) % MOD;
    ans = (ans - comb(n, a)) % MOD;
    ans = (ans - comb(n, b)) % MOD;
    if ans < 0 {
        ans += MOD;
    }

    return format!("{}", ans);
}

const MOD: i64 = 1000000007;

fn pow(n: i64, x: i64) -> i64 {
    if x == 0 {
        return 1;
    }

    return if x % 2 == 0 {
        let t = pow(n, x / 2);
        t * t % MOD
    } else {
        n * pow(n, x - 1) % MOD
    }
}

fn comb(n: i64, k: i64) -> i64 {
    let mut numerator: i64 = 1;
    let mut denominator: i64 = 1;
    for i in 0..k {
        numerator = numerator * (n - i) % MOD;
        denominator = denominator * (k - i) % MOD;
    }
    numerator * pow(denominator, MOD - 2) % MOD
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4 1 3",
            "7"
        ),
        (
            r"1000000000 141421 173205",
            "34076506"
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
