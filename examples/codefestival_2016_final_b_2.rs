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

    let mut sum = 0;
    let mut ans: Vec<usize> = Vec::new();
    for i in 1..(n + 1) {
        sum += i;
        ans.push(i);
        if sum >= n {
            break;
        }
    }

    if sum > n {
        ans.remove(sum - n - 1);
    }

    let mut result = String::new();
    for a in ans {
        result.push_str(&format!("{}\n", a));
    }
    return result.trim().to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4",
            "1
3"
        ),
        (
            r"7",
            "1
2
4"
        ),
        (
            r"1",
            "1"
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