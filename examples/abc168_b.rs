use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let k: usize = iterator.next().unwrap().parse().unwrap();
    let s: Vec<char> = iterator.next().unwrap().chars().collect();

    if s.len() <= k {
        return s.iter().map(|it| it.to_string()).collect::<Vec<String>>().join("")
    }

    let mut result = String::new();
    for i in 0..k {
        result.push(s[i]);
    }
    result.push_str("...");
    return result;
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"7
nikoandsolstice",
            "nikoand..."
        ),
        (
            r"40
ferelibenterhominesidquodvoluntcredunt",
            "ferelibenterhominesidquodvoluntcredunt"
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