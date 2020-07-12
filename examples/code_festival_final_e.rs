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
    let r: Vec<isize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    let mut a: Vec<char> = Vec::new();
    for i in 1..n {
        if r[i - 1] < r[i] {
            a.push('<');
        } else if r[i - 1] > r[i] {
            a.push('>');
        }
    }

    if a.len() <= 1 {
        return "0".to_string()
    }

    let mut b: Vec<char> = Vec::new();
    b.push(*a.first().unwrap());
    for i in 1..a.len() {
        if a[i] != a[i - 1] {
            b.push(a[i]);
        }
    }

    let ans = b.len() + 1;

    if ans < 3 {
        "0".to_string()
    } else {
        ans.to_string()
    }
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4
1 2 5 1",
            "3"
        ),
        (
            r"5
1 2 3 4 5",
            "0"
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