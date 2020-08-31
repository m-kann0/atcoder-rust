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

    let k: usize = keta(*a.iter().max().unwrap());
    let mut v: Vec<Vec<usize>> = vec![vec![0, 0]; k];
    for i in 0..n {
        let mut ai = a[i];
        for j in (0..k).rev() {
            v[j][ai % 2] += 1;
            ai /= 2;
        }
    }

    const MOD: usize = 1_000_000_007;
    let mut ans: usize = 0;
    let mut x = 1;
    for i in (0..k).rev() {
        ans += (((v[i][0] * v[i][1]) % MOD) * x) % MOD;
        ans %= MOD;
        x = (x * 2) % MOD;
    }
    ans.to_string()
}

fn keta(mut x: usize) -> usize {
    if x == 0 {
        return 0;
    }
    let mut result = 0;
    while x > 0 {
        x /= 2;
        result += 1;
    }
    result
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3
1 2 3",
            "6"
        ),
        (
            r"10
3 1 4 1 5 9 2 6 5 3",
            "237"
        ),
        (
            r"10
3 14 159 2653 58979 323846 2643383 27950288 419716939 9375105820",
            "103715602"
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