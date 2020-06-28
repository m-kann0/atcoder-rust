use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let a: Vec<usize> = (0..5).map(|_| iterator.next().unwrap().parse().unwrap()).collect();
    let k: usize = iterator.next().unwrap().parse().unwrap();

    for i in 0..5 {
        for j in 0..5 {
            if i >= j {
                continue;
            }
            if a[j] - a[i] > k {
                return ":(".to_string();
            }
        }
    }

    return "Yay!".to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"1
2
4
8
9
15",
            "Yay!"
        ),
        (
            r"15
18
26
35
36
18",
            ":("
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