use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let l: usize = iterator.next().unwrap().parse().unwrap();
    let r: usize = iterator.next().unwrap().parse().unwrap();
    let d: usize = iterator.next().unwrap().parse().unwrap();

    let mut ans: usize = 0;
    for i in l..=r {
        if i % d == 0 {
            ans += 1;
        }
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5 10 2",
            "3"
        ),
        (
            r"6 20 7",
            "2"
        ),
        (
            r"1 100 1",
            "100"
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