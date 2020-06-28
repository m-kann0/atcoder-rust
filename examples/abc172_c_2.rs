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
    let m: usize = iterator.next().unwrap().parse().unwrap();
    let k: usize = iterator.next().unwrap().parse().unwrap();
    let a: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();
    let b: Vec<usize> = (0..m).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    let mut sa: Vec<usize> = vec![0; n + 1];
    for i in 0..n {
        sa[i + 1] = sa[i] + a[i];
    }

    let mut sb: Vec<usize> = vec![0; m + 1];
    for i in 0..m {
        sb[i + 1] = sb[i] + b[i];
    }

    let mut ans: usize = 0;
    for i in 0..(n + 1) {
        if sa[i] > k {
            continue;
        }
        let t = k - sa[i];
        let mut ok: isize = 0;
        let mut ng: isize = sb.len() as isize;
        while (ng - ok).abs() > 1 {
            let mid = (ok + ng) / 2;
            if sb[mid as usize] <= t {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        ans = max(ans, i + ok as usize);
    }

    return ans.to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 4 240
60 90 120
80 150 80 150",
            "3"
        ),
        (
            r"3 4 730
60 90 120
80 150 80 150",
            "7"
        ),
        (
            r"5 4 1
1000000000 1000000000 1000000000 1000000000 1000000000
1000000000 1000000000 1000000000 1000000000",
            "0"
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