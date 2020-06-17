use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let mut x =
        iterator.next().unwrap().chars().collect::<std::collections::VecDeque<char>>();

    let x_len = x.len();

    let mut count: usize = 0;
    let mut sc: usize = 0;
    while !x.is_empty() {
        let c = x.pop_front().unwrap();
        if c == 'S' {
            sc += 1;
        } else {
            if sc > 0 {
                count += 1;
                sc -= 1;
            }
        }
    }

    let ans = x_len - (count * 2);
    return ans.to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"TSTTSS",
            "4"
        ),
        (
            r"SSTTST",
            "0"
        ),
        (
            r"TSSTTTSS",
            "4"
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