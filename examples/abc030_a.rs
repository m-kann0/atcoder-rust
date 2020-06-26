use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let a: f64 = iterator.next().unwrap().parse().unwrap();
    let b: f64 = iterator.next().unwrap().parse().unwrap();
    let c: f64 = iterator.next().unwrap().parse().unwrap();
    let d: f64 = iterator.next().unwrap().parse().unwrap();

    let s = b / a;
    let t = d / c;

    return if s == t {
        "DRAW".to_string()
    } else if s > t {
        "TAKAHASHI".to_string()
    } else {
        "AOKI".to_string()
    }
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5 2 6 3",
            "AOKI"
        ),
        (
            r"100 80 100 73",
            "TAKAHASHI"
        ),
        (
            r"66 30 55 25",
            "DRAW"
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