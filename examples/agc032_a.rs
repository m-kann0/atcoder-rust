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
    let mut b: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    let mut a: Vec<usize> = Vec::new();

    while !b.is_empty() {
        let mut found = false;
        for i in (0..b.len()).rev() {
            if b[i] == i + 1 {
                a.push(b.remove(i));
                found = true;
                break;
            }
        }
        if !found {
            break;
        }
    }

    if !b.is_empty() {
        return "-1".to_string()
    }

    return a.iter().rev().map(|it| it.to_string()).collect::<Vec<String>>().join("\n");
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3
1 2 1",
            "1
1
2"
        ),
        (
            r"2
2 2",
            "-1"
        ),
        (
            r"9
1 1 1 2 2 1 2 3 2",
            "1
2
2
3
1
2
2
1
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