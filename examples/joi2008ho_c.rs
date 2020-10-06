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
    let mut p: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    let mut pp: Vec<usize> = Vec::new();
    for i in 0..n {
        for j in 0..n {
            pp.push(p[i] + p[j]);
        }
    }

    p.sort();
    pp.sort();

    // 1本投げる場合
    let mut p1: usize = 0;
    for i in 0..n {
        if p[i] <= m {
            p1 = max(p1, p[i]);
        }
    }

    // 2本投げる場合
    let mut p2: usize = 0;
    for i in 0..n {
        for j in 0..n {
            if p[i] + p[j] <= m {
                p2 = max(p2, p[i] + p[j]);
            }
        }
    }

    // 3本投げる場合
    let mut p3: usize = 0;
    for i in 0..n {
        let s: usize = p[i];

        let mut ok: isize = -1;
        let mut ng: isize = pp.len() as isize;
        while (ok - ng).abs() > 1 {
            let mid = (ok + ng) / 2;
            if pp[mid as usize] + s <= m {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        if ok != -1 {
            p3 = max(p3, s + pp[ok as usize]);
        }
    }

    // 4本投げる場合
    let mut p4: usize = 0;
    for i in 0..n {
        for j in 0..n {
            let s: usize = p[i] + p[j];

            let mut ok: isize = -1;
            let mut ng: isize = pp.len() as isize;
            while (ok - ng).abs() > 1 {
                let mid = (ok + ng) / 2;
                if pp[mid as usize] + s <= m {
                    ok = mid;
                } else {
                    ng = mid;
                }
            }
            if ok != -1 {
                p4 = max(p4, s + pp[ok as usize]);
            }
        }
    }

    let ans = max(p1, max(p2, max(p3, p4)));
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4 50
            3 14 15 9",
            r"48"
        ),
        (
            r"3 21
            16 11 2",
            r"20"
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