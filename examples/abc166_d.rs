use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let x: isize = iterator.next().unwrap().parse().unwrap();

    for a in 0..1000 {
        for b in 0..1000 {
            let a5: isize = a * a * a * a * a;
            let b5: isize = b * b * b * b * b;
            if a5 - b5 == x {
                return format!("{} {}", a, b);
            }
            if (-a5) - b5 == x {
                return format!("{} {}", -a, b);
            }
            if a5 - (-b5) == x {
                return format!("{} {}", a, -b);
            }
            if (-a5) - (-b5) == x {
                return format!("{} {}", -a, -b);
            }
        }
    }

    return String::new();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"33",
            "2 -1"
        ),
        (
            r"1",
            "0 -1"
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