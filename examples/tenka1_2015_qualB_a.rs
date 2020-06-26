use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(_input: &str) -> String {
    return prize(19).to_string();
}

fn prize(n: usize) -> usize {
    if n == 0 || n == 1 {
        return 100;
    }
    if n == 2 {
        return 200;
    }
    return prize(n - 1) + prize(n - 2) + prize(n - 3);
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![

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