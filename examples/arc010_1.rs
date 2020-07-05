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
    let m: usize = iterator.next().unwrap().parse().unwrap();
    let a: usize = iterator.next().unwrap().parse().unwrap();
    let b: usize = iterator.next().unwrap().parse().unwrap();

    for i in 1..=m {
        if n <= a {
            n += b;
        }
        let ci: usize = iterator.next().unwrap().parse().unwrap();
        if ci > n {
            return i.to_string();
        }
        n -= ci;
    }

    return "complete".to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"100 3 0 100
10
20
30",
            r"complete"
        ),
        (
            r"100 4 0 100
10
20
30
40",
            r"complete"
        ),
        (
            r"100 4 0 100
50
40
30
20",
            r"3"
        ),
        (
            r"100 4 10 100
50
40
30
20",
            r"complete"
        ),
        (
            r"5 3 20 10
15
5
20",
            r"3"
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