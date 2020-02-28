use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: u64 = iterator.next().unwrap().parse().unwrap();
    let m: usize = iterator.next().unwrap().parse().unwrap();

    let mut prev: i64 = -1;
    let mut vec: Vec<i64> = Vec::with_capacity(m + 2);
    vec.push(-1);
    for _i in 0..m {
        let current = iterator.next().unwrap().parse().unwrap();
        if prev + 1 == current {
            // 連続したら到達不可能
            return String::from("0");
        }
        vec.push(current);
        prev = current;
    }
    vec.push((n + 1) as i64);

    let mut ans: u64 = 1;
    let mut memo: Vec<u64> = vec![0; 10_001];
    for i in 1..(m + 2) {
        ans *= fib((vec[i] - vec[i - 1] - 2) as usize, &mut memo) % MOD;
    }
    ans %= MOD;

    return ans.to_string();
}

const MOD: u64 = 1_000_000_007;

fn fib(n: usize, memo: &mut Vec<u64>) -> u64 {
    if memo[n] != 0 {
        return memo[n];
    }

    if n <= 1 {
        memo[n] = 1;
        return 1;
    }

    let result = (fib(n - 2, memo) + fib(n - 1, memo));
    memo[n] = result;
    result
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"6 1
3",
            "4"
        ),
        (
            r"10 2
4
5",
            "0"
        ),
        (
            r"100 5
1
23
45
67
89",
            "608200469"
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