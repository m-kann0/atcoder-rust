use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let s: usize = iterator.next().unwrap().parse().unwrap();

    let mut ok: isize = 1_000_000_000;
    let mut ng: isize = 0;
    while (ok - ng).abs() > 1 {
        let mid = (ok + ng) / 2;
        if mid * mid >= s as isize {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    let w: usize = ok as usize;
    let h: usize = if w * (w - 1) >= s {
        w - 1
    } else {
        w
    };

    let mut x1: usize = w;
    let y1: usize = 0;
    let x2: usize = 0;
    let mut y2: usize = 0;
    let x3 = w;
    let y3 = h;

    let mut current = w * h;
    if current == s {
        return format!("{} {} {} {} {} {}", x1, y1, x2, y2, x3, y3);
    }

    x1 -= 1;
    current -= h;
    y2 = s - current;

    return format!("{} {} {} {} {} {}", x1, y1, x2, y2, x3, y3);
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3",
            "1 0 2 2 0 1"
        ),
        (
            r"100",
            "0 0 10 0 0 10"
        ),
        (
            r"311114770564041497",
            "314159265 358979323 846264338 327950288 419716939 937510582"
        ),
        (
            r"1000000000000000000",
            "1000000000 0 0 0 1000000000 1000000000"
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