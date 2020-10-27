use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let _n: usize = iterator.next().unwrap().parse().unwrap();
    let m: usize = iterator.next().unwrap().parse().unwrap();
    let mut bridges: Vec<(usize, usize)> = Vec::new();
    for _ in 0..m {
        let left: usize = iterator.next().unwrap().parse().unwrap();
        let right: usize = iterator.next().unwrap().parse().unwrap();
        bridges.push((right, left));
    };

    bridges.sort();

    let mut ans: usize = 0;
    let mut prev = 0;
    for i in 0..m {
        let (right, left) = bridges[i];
        if left >= prev {
            ans += 1;
            prev = right;
        }
    };
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5 2
1 4
2 5",
            "1"
        ),
        (
            r"9 5
1 8
2 7
3 5
4 6
7 9",
            "2"
        ),
        (
            r"5 10
1 2
1 3
1 4
1 5
2 3
2 4
2 5
3 4
3 5
4 5",
            "4"
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