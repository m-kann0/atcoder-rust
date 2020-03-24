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
    let x: usize = iterator.next().unwrap().parse().unwrap();
    let a: Vec<usize> = (0..n)
        .map(|_| iterator.next().unwrap().parse().unwrap())
        .collect::<Vec<usize>>();

    let b = format!("{:0>1$b}", x, n);

    let ans: usize = b.chars().rev().zip(a)
        .map(|(bit, price)| {
            if bit == '1' {
                price
            } else {
                0
            }
        })
        .sum();

    return ans.to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4 5
1 10 100 1000",
            "101"
        ),
        (
            r"20 1048575
1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20",
            "210"
        ),
        (
            r"4 0
1000 1000 1000 1000",
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