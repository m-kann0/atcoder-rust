use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let mut a: Vec<(usize, char)> = Vec::new();
    a.push((iterator.next().unwrap().parse().unwrap(), 'A'));
    a.push((iterator.next().unwrap().parse().unwrap(), 'B'));
    a.push((iterator.next().unwrap().parse().unwrap(), 'C'));
    a.sort();
    a[1].1.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"15 49 7",
            "A"
        ),
        (
            r"53 2 1",
            "B"
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