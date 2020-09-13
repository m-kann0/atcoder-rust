use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let s: usize = iterator.next().unwrap().parse().unwrap();
    if s <= 2 {
        return "0".to_string();
    }

    const MOD: usize = 1_000_000_007;
    let mut dp: Vec<usize> = vec![0; s + 1];
    dp[0] = 1;
    for i in 3..=s {
        for j in 0..=(i - 3) {
            dp[i] += dp[j];
            dp[i] %= MOD;
        }
    }
    dp[s].to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"7",
            "3"
        ),
        (
            r"2",
            "0"
        ),
        (
            r"1729",
            "294867501"
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