use std::io::Read;

const MOD: usize = 1_000_000_007;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let k: usize = iterator.next().unwrap().parse().unwrap();
    let a: Vec<usize> = (0..n)
        .map(|_| iterator.next().unwrap().parse().unwrap())
        .collect();

    // b[i] = iの個数
    let mut b: Vec<usize> = vec![0; 2002];
    for &ai in a.iter() {
        b[ai] += 1;
    }

    // c[i] = iより大きい数の個数
    let mut c: Vec<usize> = vec![0; 2002];
    for i in (1..2001).rev() {
        c[i] = b[i + 1] + c[i + 1];
    }

    // 整数列A単体での個数
    let mut t = 0;
    for i in 1..n {
        for j in 0..i {
            if a[j] > a[i] {
                t += 1;
            }
        }
    }

    let mut ans = t * k;
    ans %= MOD;

    for i in 0..2001 {
        ans = (ans + b[i] * (c[i] * ((k * (k - 1) / 2) % MOD) % MOD) % MOD) % MOD;
    }

    return ans.to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2 2
2 1",
            "3"
        ),
        (
            r"3 5
1 1 1",
            "0"
        ),
        (
            r"10 998244353
10 9 8 7 5 6 3 4 2 1",
            "185297239"
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