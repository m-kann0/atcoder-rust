use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: isize = iterator.next().unwrap().parse().unwrap();
    let l: isize = iterator.next().unwrap().parse().unwrap();

    let mut t: Vec<isize> = Vec::new();
    for i in 1..=n {
        t.push(l + i - 1);
    }
    t.sort_by_key(|it| it.abs());

    let m = t.first().unwrap();
    let ans: isize = t.iter().sum::<isize>() - *m;
    return ans.to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5 2",
            "18"
        ),
        (
            r"3 -1",
            "0"
        ),
        (
            r"30 -50",
            "-1044"
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