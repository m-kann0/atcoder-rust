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
    let mut v: Vec<(usize, usize)> = Vec::with_capacity(n);
    for i in 0..n {
        let a: usize = iterator.next().unwrap().parse().unwrap();
        v.push((i + 1, a));
    }

    v.sort_by(|a, b| a.1.cmp(&b.1));

    let mut result = String::new();
    for &(i, _) in v.iter().rev() {
        result.push_str(&format!("{}\n", i));
    }
    return result.trim().to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3
140 180 160",
            r"2
3
1"
        ),
        (
            r"2
1000000000 1",
            r"1
2"
        ),
        (
            r"8
3 1 4 15 9 2 6 5",
            r"4
5
7
8
3
1
6
2"
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