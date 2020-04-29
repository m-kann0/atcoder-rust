use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let mut n: isize = iterator.next().unwrap().parse().unwrap();
    let ng1: isize = iterator.next().unwrap().parse().unwrap();
    let ng2: isize = iterator.next().unwrap().parse().unwrap();
    let ng3: isize = iterator.next().unwrap().parse().unwrap();

    let mut count = 0;
    while n > 0 {
        if n == ng1 || n == ng2 || n == ng3 {
            return "NO".to_string();
        }
        if n - 3 != ng1 && n - 3 != ng2 && n - 3 != ng3 {
            n -= 3;
        } else if n - 2 != ng1 && n - 2 != ng2 && n - 2 != ng3 {
            n -= 2;
        } else {
            n -= 1;
        }
        count += 1;
    }

    return if count > 100 {
        "NO".to_string()
    } else {
        "YES".to_string()
    }
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2
1
7
15",
            "YES"
        ),
        (
            r"5
1
4
2",
            "YES"
        ),
        (
            r"300
57
121
244",
            "NO"
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