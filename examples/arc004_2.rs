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
    let d: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    let d_sum: usize = d.iter().sum();
    let d_max: usize = *d.iter().max().unwrap();

    let a_max = d_sum;
    let a_min = if n == 1 {
        d_sum
    } else if d_max > (d_sum - d_max) {
        d_max - (d_sum - d_max)
    } else {
        0
    };

    return format!("{}\n{}", a_max, a_min);
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"1
1024",
            r"1024
1024"
        ),
        (
            r"3
3
4
5",
            r"12
0"
        ),
        (
            r"2
512
512",
            r"1024
0"
        ),
        (
            r"3
4
8
1",
            r"13
3"
        ),
        (
            r"10
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
            r"55
0"
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