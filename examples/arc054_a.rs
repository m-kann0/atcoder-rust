use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let l: usize = iterator.next().unwrap().parse().unwrap();
    let x: usize = iterator.next().unwrap().parse().unwrap();
    let y: usize = iterator.next().unwrap().parse().unwrap();
    let s: usize = iterator.next().unwrap().parse().unwrap();
    let d: usize = iterator.next().unwrap().parse().unwrap();

    let dist_forward = if d >= s {
        d - s
    } else {
        l + d - s
    };
    let dist_backward = if d >= s {
        l + s - d
    } else {
        s - d
    };

    let mut ans = dist_forward as f64 / ((x + y) as f64);

    if y > x {
        let ans2 = dist_backward as f64 / ((y - x) as f64);
        if ans2 < ans {
            ans = ans2;
        }
    }

    return format!("{:.10}", ans);
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"6 2 3 1 5",
            "0.8000000000"
        ),
        (
            r"6 2 10 1 5",
            "0.2500000000"
        ),
        (
            r"6 3 1 5 3",
            "1.0000000000"
        ),
        (
            r"10 7 7 6 0",
            "0.2857142857"
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