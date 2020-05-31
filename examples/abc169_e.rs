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
    let mut a: Vec<usize> = Vec::new();
    let mut b: Vec<usize> = Vec::new();
    for _ in 0..n {
        let ai = iterator.next().unwrap().parse().unwrap();
        let bi = iterator.next().unwrap().parse().unwrap();
        a.push(ai);
        b.push(bi);
    }

    a.sort();
    b.sort();

    if n % 2 == 1 {
        let med_a = a[n / 2];
        let med_b = b[n / 2];
        return format!("{}", med_b - med_a + 1);
    }

    let med_a = a[n / 2 - 1] + a[n / 2];
    let med_b = b[n / 2 - 1] + b[n / 2];
    return format!("{}", med_b - med_a + 1);
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2
1 2
2 3",
            "3"
        ),
        (
            r"3
100 100
10 10000
1 1000000000",
            "9991"
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