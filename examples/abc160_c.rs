use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let k: usize = iterator.next().unwrap().parse().unwrap();
    let n: usize = iterator.next().unwrap().parse().unwrap();

    let a: Vec<isize> = (0..n)
        .map(|_| iterator.next().unwrap().parse().unwrap())
        .collect::<Vec<isize>>();

    let mut diffs: Vec<isize> = vec![0; n];
    for i in 0..(n - 1) {
        diffs[i] = a[i + 1] - a[i];
    }

    diffs[n - 1] = a[0] + (k as isize - a[n - 1]);

    diffs.sort();
    diffs.pop();

    let ans: isize = diffs.iter().sum();

    return ans.to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"20 3
5 10 15",
            "10"
        ),
        (
            r"20 3
0 5 15",
            "10"
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