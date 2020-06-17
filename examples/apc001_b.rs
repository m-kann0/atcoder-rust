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
    let a: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();
    let b: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    let a_sum: usize = a.iter().sum();
    let b_sum: usize = b.iter().sum();
    if a_sum > b_sum {
        return "No".to_string();
    }
    let c = b_sum - a_sum;

    let mut c1: usize = 0;
    let mut c2: usize = 0;
    for i in 0..n {
        if a[i] > b[i] {
            c1 += a[i] - b[i];
        }
        if b[i] > a[i] {
            c2 += (b[i] - a[i] + 1) / 2;
        }
    }
    if c1 > c || c2 > c {
        return "No".to_string();
    }

    return "Yes".to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3
1 2 3
5 2 2",
            "Yes"
        ),
        (
            r"5
3 1 4 1 5
2 7 1 8 2",
            "No"
        ),
        (
            r"5
2 7 1 8 2
3 1 4 1 5",
            "No"
        ),
        (
            r"2
1 9
5 7",
            "Yes"
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