use std::io::Read;
use std::cmp::max;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let mut a: Vec<isize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();
    a.sort();

    if n % 2 == 0 {
        let mut b: Vec<isize> = vec![-1, 1];
        for _ in 0..((n - 2) / 2) {
            b.push(2);
            b.push(-2);
        }
        b.sort();

        let mut ans = 0;
        for i in 0..n {
            ans += a[i] * b[i];
        }
        return ans.to_string();
    }

    let mut b1: Vec<isize> = vec![-1, -1];
    let mut b2: Vec<isize> = vec![1, 1];
    for _ in 0..(n / 2) {
        b1.push(2);
        b2.push(-2);
    }
    for _ in 0..(n / 2 - 1) {
        b1.push(-2);
        b2.push(2);
    }
    b1.sort();
    b2.sort();

    let mut ans1 = 0;
    let mut ans2 = 0;
    for i in 0..n {
        ans1 += a[i] * b1[i];
        ans2 += a[i] * b2[i];
    }
    max(ans1, ans2).to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5
6
8
1
2
3",
            "21"
        ),
        (
            r"6
3
1
4
1
5
9",
            "25"
        ),
        (
            r"3
5
5
1",
            "8"
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