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
    let c: usize = iterator.next().unwrap().parse().unwrap();
    let a: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    let mut min_count: usize = 100;
    for i in 1..11 {
        for j in 1..11 {
            if i == j {
                continue;
            }
            let mut count = 0;
            for k in 0..n {
                if k % 2 == 0 {
                    if a[k] != i {
                        count += 1;
                    }
                } else {
                    if a[k] != j {
                        count += 1;
                    }
                }
            }
            min_count = std::cmp::min(min_count, count);
        }
    }

    let ans = min_count * c;
    return ans.to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 10
3
2
1",
            "10"
        ),
        (
            r"4 100
1
1
1
1",
            "200"
        ),
        (
            r"10 1000
1
2
3
4
5
6
7
8
9
10",
            "8000"
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