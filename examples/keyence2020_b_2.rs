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
    let mut arms: Vec<(isize, isize)> = Vec::new();
    for _ in 0..n {
        let x: isize = iterator.next().unwrap().parse().unwrap();
        let l: isize = iterator.next().unwrap().parse().unwrap();
        let left = x - l;
        let right = x + l;
        arms.push((right, left));
    }

    arms.sort();

    let mut ans: usize = 0;
    let mut prev: isize = -1_000_000_005;
    for i in 0..n {
        let (right, left) = arms[i];
        if left >= prev {
            ans += 1;
            prev = right;
        }
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4
2 4
4 3
9 3
100 5",
            "3"
        ),
        (
            r"2
8 20
1 10",
            "1"
        ),
        (
            r"5
10 1
2 1
4 1
6 1
8 1",
            "5"
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