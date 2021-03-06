use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let s: Vec<char> = iterator.next().unwrap().chars().collect();
    let n: usize = iterator.next().unwrap().parse().unwrap();

    for i in 0..5 {
        for j in 0..5 {
            if 5 * i + j + 1 == n {
                return format!("{}{}", s[i], s[j]);
            }
        }
    }

    return String::new();
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