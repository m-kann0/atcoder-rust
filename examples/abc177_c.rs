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
    let a: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    const MOD: usize = 1_000_000_007;

    let mut s: Vec<usize> = vec![0; n];
    s[n - 1] = a[n - 1];
    for i in (0..(n - 1)).rev() {
        s[i] = s[i + 1] + a[i];
        s[i] %= MOD;
    }

    let mut ans = 0;
    for i in 0..(n - 1) {
        ans = ans + (a[i] * s[i + 1]) % MOD;
        ans %= MOD;
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3
1 2 3",
            "11"
        ),
        (
            r"4
141421356 17320508 22360679 244949",
            "437235829"
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