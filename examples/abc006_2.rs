use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

const MOD: usize = 10007;

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: usize = iterator.next().unwrap().parse().unwrap();

    if n <= 2 {
        return "0".to_string();
    }
    if n == 3 {
        return "1".to_string();
    }

    let mut memo: Vec<usize> = vec![MOD; n + 1];
    memo[1] = 0;
    memo[2] = 0;
    memo[3] = 1;
    for i in 4..(n + 1) {
        memo[i] = memo[i - 1] + memo[i - 2] + memo[i - 3];
        memo[i] %= MOD;
    }

    return memo[n].to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        ("7", "7"),
        ("1", "0"),
        ("100000", "7927"),
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