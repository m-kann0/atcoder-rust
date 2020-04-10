use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let s: i32 = iterator.next().unwrap().parse().unwrap();
    let l: i32 = iterator.next().unwrap().parse().unwrap();
    let r: i32 = iterator.next().unwrap().parse().unwrap();

    let ans = if l <= s && s <= r {
        s
    } else if s < l {
        l
    } else {
        r
    };
    return ans.to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5 1 5",
            "5"
        ),
        (
            r"2 7 10",
            "7"
        ),
        (
            r"20 3 5",
            "5"
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