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
    let p: usize = iterator.next().unwrap().parse().unwrap();

    let mut is_all_even: bool = true;
    for _ in 0..n {
        let a: usize = iterator.next().unwrap().parse().unwrap();
        if a % 2 == 1 {
            is_all_even = false;
            break;
        }
    }

    if is_all_even {
        if p == 0 {
            format!("{}", 2_u64.pow(n as u32))
        } else {
            format!("{}", 0)
        }
    } else {
        format!("{}", 2_u64.pow((n - 1) as u32))
    }
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2 0
1 3",
            "2"
        ),
        (
            r"1 1
50",
            "0"
        ),
        (
            r"3 0
1 1 1",
            "4"
        ),
        (
            r"45 1
17 55 85 55 74 20 90 67 40 70 39 89 91 50 16 24 14 43 24 66 25 9 89 71 41 16 53 13 61 15 85 72 62 67 42 26 36 66 4 87 59 91 4 25 26",
            "17592186044416"
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