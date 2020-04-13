use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let a: usize = iterator.next().unwrap().parse().unwrap();
    let b: usize = iterator.next().unwrap().parse().unwrap();

    let mut ans: usize = 0;
    let mut year = if a % 4 == 0 {
        a
    } else {
        a + (4 - a % 4)
    };
    while year <= b {
        if is_leap_year(year) {
            ans += 1;
        }
        year += 4;
    }
    return ans.to_string();
}

fn is_leap_year(year: usize) -> bool {
    if year % 400 == 0 {
        return true;
    }
    if year % 100 == 0 {
        return false;
    }
    return year % 4 == 0;
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"1988 2014",
            "7"
        ),
        (
            r"997 1003",
            "0"
        ),
        (
            r"1 2000000000",
            "485000000"
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