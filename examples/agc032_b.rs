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

    let mut result = Vec::new();
    for i in 1..=n {
        for j in 1..=n {
            if i >= j {
                continue;
            }
            if n % 2 == 0 {
                if i + j != n + 1 {
                    result.push((i, j));
                }
            } else {
                if i + j != n {
                    result.push((i, j));
                }
            }
        }
    }

    let mut ans = String::new();
    ans.push_str(&format!("{}\n", result.len()));
    for &(i, j) in &result {
        ans.push_str(&format!("{} {}\n", i, j));
    }
    ans.trim().to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3",
            "2
1 3
2 3"
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
