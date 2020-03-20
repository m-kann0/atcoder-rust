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

    let mut vec: Vec<(usize, usize)> = vec![(0, 0); m + 1];
    for _ in 0..n {
        let a: usize = iterator.next().unwrap().parse().unwrap();
        let e = vec[a];
        vec[a] = (a, e.1 + 1);
    }
    vec.sort_by_key(|e| e.1);

    if vec[m].1 == vec[m - 1].1 {
        return String::from("?");
    }

    if vec[m].1 > (n / 2) {
        return vec[m].0.to_string();
    }
    return String::from("?");
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 2
2 1 2",
            "2"
        ),
        (
            r"4 2
2 1 2 1",
            "?"
        ),
        (
            r"10 1
0 0 0 0 0 0 1 1 1 1",
            "0"
        ),
        (
            r"10 5
0 1 2 3 4 5 5 5 5 5",
            "?"
        ),
        (
            r"3 2
2 1 2",
            "2"
        ),
        (
            r"4 2
2 1 2 1",
            "?"
        ),
        (
            r"10 1
0 0 0 0 0 0 1 1 1 1",
            "0"
        ),
        (
            r"10 5
0 1 2 3 4 5 5 5 5 5",
            "?"
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