use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let mut n: usize = iterator.next().unwrap().parse().unwrap();

    let mut cards: Vec<char> = vec!['1', '2', '3', '4', '5', '6'];

    n = n % 30;
    for i in 0..n {
        let i_mod_5 = i % 5;
        cards.swap(i_mod_5, i_mod_5 + 1);
    }

    return cards.into_iter().collect();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (r"1", r"213456"),
        (r"5", r"234561"),
        (r"22", r"615234"),
        (r"30", r"123456"),
        (r"100000000", r"345612"),
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