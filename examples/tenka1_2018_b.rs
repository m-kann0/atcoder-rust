use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let mut a: usize = iterator.next().unwrap().parse().unwrap();
    let mut b: usize = iterator.next().unwrap().parse().unwrap();
    let k: usize = iterator.next().unwrap().parse().unwrap();

    for i in 0..k {
        if i % 2 == 0 {
            if a % 2 == 1 {
                a -= 1;
            }
            a /= 2;
            b += a;
        } else {
            if b % 2 == 1 {
                b -= 1;
            }
            b /= 2;
            a += b;
        }
    }

    return format!("{} {}", a, b);
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5 4 2",
            "5 3"
        ),
        (
            r"3 3 3",
            "1 3"
        ),
        (
            r"314159265 358979323 84",
            "448759046 224379523"
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