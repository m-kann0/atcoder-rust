use std::io::Read;
use std::cmp::max;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let mut h: isize = iterator.next().unwrap().parse().unwrap();

    let mut a_max: isize = 0;
    let mut b: Vec<isize> = Vec::new();
    for _ in 0..n {
        let ai: isize = iterator.next().unwrap().parse().unwrap();
        let bi: isize = iterator.next().unwrap().parse().unwrap();
        a_max = max(a_max, ai);
        b.push(bi);
    }
    b.sort();
    b.reverse();

    let mut count: usize = 0;
    for bi in b {
        if h <= 0 {
            break;
        }
        if bi < a_max {
            break;
        }
        h -= bi;
        count += 1;
    }

    if h <= 0 {
        return count.to_string();
    }

    if h % a_max == 0 {
        count += (h / a_max) as usize;
    } else {
        count += (h / a_max) as usize + 1;
    }

    return count.to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"1 10
3 5",
            "3"
        ),
        (
            r"2 10
3 5
2 6",
            "2"
        ),
        (
            r"4 1000000000
1 1
1 10000000
1 30000000
1 99999999",
            "860000004"
        ),
        (
            r"5 500
35 44
28 83
46 62
31 79
40 43",
            "9"
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