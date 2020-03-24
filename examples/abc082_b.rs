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
    let t = iterator.next().unwrap();

    let mut vec_s = s.chars().collect::<Vec<char>>();
    let mut vec_t = t.chars().collect::<Vec<char>>();

    vec_s.sort();

    vec_t.sort();
    vec_t.reverse();

    let s2 = vec_s.iter().map(|c| c.to_string()).collect::<Vec<String>>().join("");
    let t2 = vec_t.iter().map(|c| c.to_string()).collect::<Vec<String>>().join("");

    if s2 < t2 {
        "Yes".to_string()
    } else {
        "No".to_string()
    }
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"yx
axy",
            "Yes"
        ),
        (
            r"ratcode
atlas",
            "Yes"
        ),
        (
            r"cd
abc",
            "No"
        ),
        (
            r"w
ww",
            "Yes"
        ),
        (
            r"zzz
zzz",
            "No"
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