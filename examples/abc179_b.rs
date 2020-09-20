use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let mut count = 0;
    let mut is_yes = false;
    for _ in 0..n {
        let d1: usize = iterator.next().unwrap().parse().unwrap();
        let d2: usize = iterator.next().unwrap().parse().unwrap();
        if d1 == d2 {
            count += 1;
            if count >= 3 {
                is_yes = true;
                break;
            }
        } else {
            count = 0;
        }
    }

    if is_yes {
        "Yes".to_string()
    } else {
        "No".to_string()
    }
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5
1 2
6 6
4 4
3 3
3 2",
            "Yes"
        ),
        (
            r"5
1 2
6 3
4 4
3 3
3 3",
            "Yes"
        ),
        (
            r"5
1 1
2 2
3 4
5 5
6 6",
            "No"
        ),
        (
            r"6
1 1
2 2
3 3
4 4
5 5
6 6",
            "Yes"
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