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
    let a: Vec<isize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    let mut ans: isize = -2500;
    for i in 0..n {
        let mut j_max = 0;
        let mut aoki_max: isize = -2500;
        for j in 0..n {
            if i == j {
                continue;
            }
            let (l, r) = if i < j {
                (i, j)
            } else {
                (j, i)
            };
            let mut aoki = 0;
            for k in l..=r {
                if (k - l) % 2 == 1 {
                    aoki += a[k];
                }
            }
            if aoki > aoki_max {
                aoki_max = aoki;
                j_max = j;
            }
        }
        let (l, r) = if i < j_max {
            (i, j_max)
        } else {
            (j_max, i)
        };
        let mut takahashi = 0;
        for k in l..=r {
            if (k - l) % 2 == 0 {
                takahashi += a[k];
            }
        }
        ans = max(ans, takahashi);
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"6
1 -3 3 9 1 6",
            "6"
        ),
        (
            r"3
5 5 5",
            "10"
        ),
        (
            r"8
-1 10 -1 2 -1 10 -1 0",
            "-1"
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