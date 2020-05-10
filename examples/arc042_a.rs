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
    let m: usize = iterator.next().unwrap().parse().unwrap();
    let mut a: Vec<usize> = Vec::new();
    for _ in 0..m {
        a.push(iterator.next().unwrap().parse().unwrap());
    }

    let mut result = String::new();
    let mut before_write: Vec<bool> = vec![true; n];
    while !a.is_empty() {
        let ai = a.pop().unwrap();
        if before_write[ai - 1] {
            result.push_str(&format!("{}\n", ai));
            before_write[ai - 1] = false;
        }
    }
    for i in 1..(n + 1) {
        if before_write[i - 1] {
            result.push_str(&format!("{}\n", i));
        }
    }
    return result.trim().to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 3
2
3
1",
            "1
3
2"
        ),
        (
            r"3 3
1
1
1",
            "1
2
3"
        ),
        (
            r"10 10
3
1
4
1
5
9
2
6
5
3",
            "3
5
6
2
9
1
4
7
8
10"
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