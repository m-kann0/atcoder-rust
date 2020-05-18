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
    let k: usize = iterator.next().unwrap().parse().unwrap();
    let m: usize = iterator.next().unwrap().parse().unwrap();
    let r: usize = iterator.next().unwrap().parse().unwrap();
    let mut s: Vec<usize> = (0..(n - 1))
        .map(|_| iterator.next().unwrap().parse().unwrap())
        .collect();

    s.sort();
    s.reverse();
    let mut sum: usize = s.iter().take(k).sum();

    if k < n && sum >= r * k {
        return "0".to_string();
    }
    if k < n {
        sum -= s[k - 1];
    }

    let mut ng: isize = -1;
    let mut ok: isize = (m + 1) as isize;
    while (ok - ng).abs() > 1 {
        let mid = (ok + ng) / 2;
        if (sum + mid as usize) >= r * k {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    return if ok <= m as isize {
        ok.to_string()
    } else {
        "-1".to_string()
    }
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5 3 100 60
86
23
49
39",
            "45"
        ),
        (
            r"5 3 100 60
92
100
95
99",
            "0"
        ),
        (
            r"5 3 100 60
18
42
29
31",
            "-1"
        ),
        (
            r"13 10 1000000000 645245296
492014535
611893452
729291030
392019922
293849201
474839528
702912832
341845861
102495671
908590572
812912432
129855439",
            "986132796"
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