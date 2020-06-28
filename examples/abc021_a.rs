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

    let mut ans: Vec<usize> = Vec::new();
    while n > 0 {
        if n >= 8 {
            ans.push(8);
            n -= 8;
        } else if n >= 4 {
            ans.push(4);
            n -= 4;
        } else if n >= 2 {
            ans.push(2);
            n -= 2;
        } else {
            ans.push(1);
            n -= 1;
        }
    }

    let mut result = String::new();
    result.push_str(&format!("{}\n", ans.len()));
    for a in ans {
        result.push_str(&format!("{}\n", a));
    }
    return result.trim().to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5",
            "3
1
2
2"
        ),
        (
            r"1",
            "1
1"
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