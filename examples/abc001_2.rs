use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let m: usize = iterator.next().unwrap().parse().unwrap();

    let mut ans = 0;

    if m < 100 {
        ans = 0;
    }

    if m >= 100 && m <= 5000 {
        ans = m / 100;
    }

    if m >= 6000 && m <= 30000 {
        ans = m / 1000 + 50;
    }

    if m >= 35000 && m <= 70000 {
        ans = (m / 1000 - 30) / 5 + 80;
    }

    if m > 70000 {
        ans = 89;
    }

    return if ans < 10 {
        format!("0{}", ans)
    } else {
        ans.to_string()
    }
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (r"15000", r"65"),
        (r"75000", r"89"),
        (r"200", r"02"),
        (r"0", r"00"),
        (r"99", r"00"),
        (r"100", r"01"),
        (r"5000", r"50"),
        (r"6000", r"56"),
        (r"30000", r"80"),
        (r"35000", r"81"),
        (r"70000", r"88"),
        (r"70001", r"89"),
        (r"100000", r"89"),
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