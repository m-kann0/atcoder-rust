use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let x = iterator.next().unwrap().chars().next().unwrap();
    let y = iterator.next().unwrap().chars().next().unwrap();

    let a = vec!['A', 'B', 'C', 'D', 'E', 'F'];

    let xp = a.iter().position(|c| *c == x).unwrap();
    let yp = a.iter().position(|c| *c == y).unwrap();

    if xp < yp {
        "<".to_string()
    } else if xp > yp {
        ">".to_string()
    } else {
        "=".to_string()
    }
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"A B",
            "<"
        ),
        (
            r"E C",
            ">"
        ),
        (
            r"F F",
            "="
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