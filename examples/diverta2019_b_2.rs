use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let r: isize = iterator.next().unwrap().parse().unwrap();
    let g: isize = iterator.next().unwrap().parse().unwrap();
    let b: isize = iterator.next().unwrap().parse().unwrap();
    let n: isize = iterator.next().unwrap().parse().unwrap();

    let mut ans: usize = 0;
    for ri in 0..=3000 {
        for gi in 0..=3000 {
            let rest = n - ri * r - gi * g;
            if rest >= 0 && rest % b == 0 {
                ans += 1;
            }
        }
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"1 2 3 4",
            "4"
        ),
        (
            r"13 1 4 3000",
            "87058"
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