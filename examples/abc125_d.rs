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
    let mut abs_array: Vec<isize> = Vec::new();
    let mut minus_count: usize = 0;
    for _ in 0..n {
        let a: isize = iterator.next().unwrap().parse().unwrap();
        if a < 0 {
            minus_count += 1;
            abs_array.push(-a);
        } else {
            abs_array.push(a);
        }
    }

    if minus_count % 2 == 0 {
        let ans: isize = abs_array.iter().sum();
        return ans.to_string();
    }

    abs_array.sort();

    let mut ans: isize = -abs_array[0];
    for i in 1..n {
        ans += abs_array[i];
    }
    return ans.to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3
-10 5 -4",
            "19"
        ),
        (
            r"5
10 -4 -8 -11 3",
            "30"
        ),
        (
            r"11
-1000000000 1000000000 -1000000000 1000000000 -1000000000 0 1000000000 -1000000000 1000000000 -1000000000 1000000000",
            "10000000000"
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