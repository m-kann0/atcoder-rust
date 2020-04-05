use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let w: usize = iterator.next().unwrap().parse().unwrap();
    let h: usize = iterator.next().unwrap().parse().unwrap();
    let x: usize = iterator.next().unwrap().parse().unwrap();
    let y: usize = iterator.next().unwrap().parse().unwrap();

    let area: f64 = w as f64 * h as f64 / 2f64;

    let multi: char = if x * 2 == w && y * 2 == h {
        '1'
    } else {
        '0'
    };

    return format!("{} {}", area, multi);
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2 3 1 2",
            "3.000000 0"
        ),
        (
            r"2 2 1 1",
            "2.000000 1"
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