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
    let k: usize = iterator.next().unwrap().parse().unwrap();
    let mut a: Vec<bool> = vec![false; n];
    for _ in 0..k {
        let d: usize = iterator.next().unwrap().parse().unwrap();
        for _ in 0..d {
            let ai: usize = iterator.next().unwrap().parse().unwrap();
            a[ai - 1] = true;
        }
    }

    let ans: usize = a.iter().filter(|it| **it == false).count();
    return ans.to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 2
2
1 3
1
3",
            "1"
        ),
        (
            r"3 3
1
3
1
3
1
3",
            "2"
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