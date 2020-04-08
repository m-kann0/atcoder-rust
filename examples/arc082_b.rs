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
    let mut indexes: Vec<usize> = Vec::new();
    for i in 0..n {
        let p: usize = iterator.next().unwrap().parse().unwrap();
        if p == (i + 1) {
            indexes.push(p);
        }
    }

    let mut count = 0;
    let mut prev = 0;
    for i in indexes {
        if prev == 0 || prev + 1 != i {
            count += 1;
            prev = i;
        } else if prev + 1 == i {
            prev = 0;
        }
    }

    return count.to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5
1 4 3 5 2",
            "2"
        ),
        (
            r"2
1 2",
            "1"
        ),
        (
            r"2
2 1",
            "0"
        ),
        (
            r"9
1 2 4 9 5 8 7 3 6",
            "3"
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