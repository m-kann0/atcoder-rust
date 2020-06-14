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
    let q: usize = iterator.next().unwrap().parse().unwrap();
    let mut sum: Vec<isize> = vec![0; n];
    for _ in 0..q {
        let l: usize = iterator.next().unwrap().parse::<usize>().unwrap() - 1;
        let r: usize = iterator.next().unwrap().parse::<usize>().unwrap() - 1;
        sum[l] += 1;
        if r + 1 <= n - 1 {
            sum[r + 1] -= 1;
        }
    }
    for i in 1..n {
        sum[i] += sum[i - 1];
    }

    let mut result = String::new();
    for i in 0..n {
        if sum[i] % 2 == 0 {
            result.push('0');
        } else {
            result.push('1');
        }
    }
    return result;
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5 4
1 4
2 5
3 3
1 5",
            "01010"
        ),
        (
            r"20 8
1 8
4 13
8 8
3 18
5 20
19 20
2 7
4 9",
            "10110000011110000000"
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
