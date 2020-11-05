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
    let mut a: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    a.sort();
    a.reverse();

    let mut score_a: usize = 0;
    let mut score_b: usize = 0;
    for i in 0..n {
        if i % 2 == 0 {
            score_a += a[i];
        } else {
            score_b += a[i];
        }
    }
    (score_a - score_b).to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2
3 1",
            "2"
        ),
        (
            r"3
2 7 4",
            "5"
        ),
        (
            r"4
20 18 2 18",
            "18"
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