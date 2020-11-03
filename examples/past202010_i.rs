use std::io::Read;
use std::cmp::min;

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

    let total: isize = a.iter().sum();
    // eprintln!("total = {:?}", total);

    let mut left: Vec<isize> = vec![0; n + 1];
    for i in 0..n {
        left[i + 1] = left[i] + a[i];
    }
    // eprintln!("left = {:?}", left);

    let mut right: Vec<isize> = vec![0; n + 1];
    for i in (0..n).rev() {
        right[i] = right[i + 1] + a[i];
    }
    right.reverse();
    // eprintln!("right = {:?}", right);

    let mut ans: isize = std::isize::MAX;
    for i in 0..(n + 1) {
        let l = left[i];
        let mut ok: isize = i as isize;
        let mut ng: isize = (n + 1) as isize;
        while (ok - ng).abs() > 1 {
            let mid = (ok + ng) / 2;
            if (left[mid as usize] - l) * 2 <= total {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        let ok = ok as usize;
        let x = left[ok] - l;
        let y = total - x;

        ans = min(ans, (x - y).abs());
        if ok + 1 < n + 1 {
            let x = left[ok + 1] - l;
            let y = total - x;
            ans = min(ans, (x - y).abs());
        }

        let mut ok: isize = 0 as isize;
        let mut ng: isize = (n + 1) as isize;
        while (ok - ng).abs() > 1 {
            let mid = (ok + ng) / 2;
            if (right[mid as usize] + l) * 2 <= total {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        let ok = ok as usize;
        let x = l + right[ok];
        let y = total - x;
        ans = min(ans, (x - y).abs());
        if ok + 1 < n + 1 {
            let x = l + right[ok + 1];
            let y = total - x;
            ans = min(ans, (x - y).abs());
        }
    }

    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4
10 20 40 30",
            "20"
        ),
        (
            r"20
13 76 46 15 50 98 93 77 31 43 84 90 6 24 14 37 73 29 43 9",
            "1"
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