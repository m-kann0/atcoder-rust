use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: u128 = iterator.next().unwrap().parse().unwrap();

    let mut x = 3;
    let mut a: usize = 1;
    while x < n {
        let mut y = 5;
        let mut b = 1;
        let m = n - x;
        while y <= m {
            if y == m {
                return format!("{} {}", a, b);
            }
            y *= 5;
            b += 1;
        }
        x *= 3;
        a += 1;
    }
    "-1".to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"106",
            "4 2"
        ),
        (
            r"1024",
            "-1"
        ),
        (
            r"10460353208",
            "21 1"
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