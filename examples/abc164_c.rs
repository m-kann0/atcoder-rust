use std::io::Read;
use std::collections::HashSet;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let mut set: HashSet<String> = HashSet::new();
    for _ in 0..n {
        set.insert(iterator.next().unwrap().to_string());
    }
    return format!("{}", set.len());
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3
apple
orange
apple",
            "2"
        ),
        (
            r"5
grape
grape
grape
grape
grape",
            "1"
        ),
        (
            r"4
aaaa
a
aaa
aa",
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