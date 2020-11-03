use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: isize = iterator.next().unwrap().parse().unwrap();
    let m: isize = iterator.next().unwrap().parse().unwrap();

    if m < 0 {
        return "-1".to_string();
    }

    if m == 0 {
        let mut result = String::new();
        for i in 0..n {
            result.push_str(&format!("{} {}\n", 2 * i + 1, 2 * i + 2));
        }
        return result.trim().to_string();
    }

    if m + 2 > n {
        return "-1".to_string();
    }

    let mut result = String::new();
    let l = 1;
    let r = (m + 1) * 2 + 2;
    result.push_str(&format!("{} {}\n", l, r));
    for i in 0..(m + 1) {
        let l = (i + 1) * 2;
        let r = l + 1;
        result.push_str(&format!("{} {}\n", l, r));
    }

    let mut l = r + 1;
    let mut r = l + 1;
    for _ in 0..(n - m - 2) {
        result.push_str(&format!("{} {}\n", l, r));
        l += 2;
        r += 2;
    }
    result.trim().to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5 1",
            "1 10
8 12
13 20
11 14
2 4"
        ),
        (
            r"10 -10",
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