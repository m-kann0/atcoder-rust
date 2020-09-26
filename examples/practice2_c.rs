use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let t: usize = iterator.next().unwrap().parse().unwrap();

    let mut result = String::new();
    for _ in 0..t {
        let n: i64 = iterator.next().unwrap().parse().unwrap();
        let m: i64 = iterator.next().unwrap().parse().unwrap();
        let a: i64 = iterator.next().unwrap().parse().unwrap();
        let b: i64 = iterator.next().unwrap().parse().unwrap();

        result.push_str(&format!("{}\n", floor_sum(n, m, a, b)));
    }
    result.trim().to_string()
}

pub fn floor_sum(n: i64, m: i64, mut a: i64, mut b: i64) -> i64 {
    let mut ans = 0;
    if a >= m {
        ans += (n - 1) * n * (a / m) / 2;
        a %= m;
    }
    if b >= m {
        ans += n * (b / m);
        b %= m;
    }

    let y_max = (a * n + b) / m;
    let x_max = y_max * m - b;
    if y_max == 0 {
        return ans;
    }
    ans += (n - (x_max + a - 1) / a) * y_max;
    ans += floor_sum(y_max, a, m, (a - x_max % a) % a);
    ans
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5
4 10 6 3
6 5 4 3
1 1 0 0
31415 92653 58979 32384
1000000000 1000000000 999999999 999999999",
            "3
13
0
314095480
499999999500000000"
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