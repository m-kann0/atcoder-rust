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
    let a: usize = iterator.next().unwrap().parse().unwrap();
    let b: usize = iterator.next().unwrap().parse().unwrap();

    if a == b {
        return if n % (a + 1) == 0 {
            "Aoki".to_string()
        } else {
            "Takahashi".to_string()
        };
    }

    if a < b {
        return if n <= a {
            "Takahashi".to_string()
        } else {
            "Aoki".to_string()
        };
    }

    return "Takahashi".to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5
3 3",
            "Takahashi"
        ),
        (
            r"4
3 3",
            "Aoki"
        ),
        (
            r"5
3 2",
            "Takahashi"
        ),
        (
            r"1000000000
1000000000 1",
            "Takahashi"
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