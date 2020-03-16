use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let s: &str = iterator.next().unwrap();

    if s.starts_with("keyence")
        || s.starts_with("keyenc") && s.ends_with("e")
        || s.starts_with("keyen") && s.ends_with("ce")
        || s.starts_with("keye") && s.ends_with("nce")
        || s.starts_with("key") && s.ends_with("ence")
        || s.starts_with("ke") && s.ends_with("yence")
        || s.starts_with("k") && s.ends_with("eyence")
        || s.ends_with("keyence")
    {
        return String::from("YES");
    }

    return String::from("NO");
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"keyofscience",
            "YES"
        ),
        (
            r"mpyszsbznf",
            "NO"
        ),
        (
            r"ashlfyha",
            "NO"
        ),
        (
            r"keyence",
            "YES"
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