use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut ans = String::new();

    for c in input.trim().chars() {
        if c == ' ' {
            if !ans.ends_with(" ") {
                ans.push(' ');
            }
        } else {
            ans.push(c);
        }
    }

    ans = ans.replace(" ", ",");

    return ans;
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (r"X Y Z", r"X,Y,Z"),
        (r"A  B, C", r"A,B,,C"),
        (r"QWERTY", r"QWERTY"),
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