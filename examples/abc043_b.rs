use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let s = iterator.next().unwrap();

    let mut ans = String::new();
    for c in s.chars() {
        if c == 'B' {
            ans.pop();
        } else {
            ans.push(c);
        }
    }

    return ans;
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"01B0",
            "00"
        ),
        (
            r"0BB1",
            "1"
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