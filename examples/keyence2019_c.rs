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

    let a: Vec<i64> = (0..n)
        .map(|_| iterator.next().unwrap().parse().unwrap())
        .collect();
    let b: Vec<i64> = (0..n)
        .map(|_| iterator.next().unwrap().parse().unwrap())
        .collect();

    let mut d_sum = 0;
    let mut minus_sum = 0;
    let mut count = 0;
    let mut plus_d: Vec<i64> = Vec::new();
    for i in 0..n {
        let d = a[i] - b[i];
        d_sum += d;
        if d < 0 {
            minus_sum += d;
            count += 1;
        } else {
            plus_d.push(d);
        }
    }

    if d_sum < 0 {
        return "-1".to_string();
    }

    plus_d.sort();

    while minus_sum < 0 {
        let d = plus_d.pop().unwrap();
        minus_sum += d;
        count += 1;
    }

    return count.to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3
2 3 5
3 4 1",
            "3"
        ),
        (
            r"3
2 3 3
2 2 1",
            "0"
        ),
        (
            r"3
17 7 1
25 6 14",
            "-1"
        ),
        (
            r"12
757232153 372327760 440075441 195848680 354974235 458054863 463477172 740174259 615762794 632963102 529866931 64991604
74164189 98239366 465611891 362739947 147060907 118867039 63189252 78303147 501410831 110823640 122948912 572905212",
            "5"
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