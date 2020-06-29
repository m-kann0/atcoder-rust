use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let a: usize = iterator.next().unwrap().parse().unwrap();
    let b: usize = iterator.next().unwrap().parse().unwrap();
    let c: usize = iterator.next().unwrap().parse().unwrap();

    let mut v = vec![a, b, c];
    v.sort();
    v.reverse();

    return format!(
        "{}\n{}\n{}",
        v.iter().position(|x| *x == a).unwrap() + 1,
        v.iter().position(|x| *x == b).unwrap() + 1,
        v.iter().position(|x| *x == c).unwrap() + 1
    );
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"12
18
11",
            "2
1
3"
        ),
        (
            r"10
20
30",
            "3
2
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