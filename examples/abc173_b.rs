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

    let mut ac: usize = 0;
    let mut wa: usize = 0;
    let mut tle: usize = 0;
    let mut re: usize = 0;
    for _ in 0..n {
        let s = iterator.next().unwrap();
        if s == "AC" {
            ac += 1;
        } else if s == "WA" {
            wa += 1;
        } else if s == "TLE" {
            tle += 1;
        } else if s == "RE" {
            re += 1;
        }
    }

    return format!("AC x {}\nWA x {}\nTLE x {}\nRE x {}", ac, wa, tle, re);
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"6
AC
TLE
AC
AC
WA
TLE",
            "AC x 3
WA x 1
TLE x 2
RE x 0"
        ),
        (
            r"10
AC
AC
AC
AC
AC
AC
AC
AC
AC
AC",
            "AC x 10
WA x 0
TLE x 0
RE x 0"
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