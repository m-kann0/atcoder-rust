use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let a: usize = iterator.next().unwrap().parse().unwrap();
    let b: usize = iterator.next().unwrap().parse().unwrap();
    let c: usize = iterator.next().unwrap().parse().unwrap();
    let d: usize = iterator.next().unwrap().parse().unwrap();
    let e: usize = iterator.next().unwrap().parse().unwrap();

    let mut dishes: Vec<usize> = vec![a, b, c, d, e];
    dishes.sort_by(|a, b| {
        let a = if a % 10 == 0 { 0 } else { 10 - a % 10 };
        let b = if b % 10 == 0 { 0 } else { 10 - b % 10 };
        a.cmp(&b)
    });
    eprintln!("dishes = {:?}", dishes);

    let mut ans: usize = 0;
    for i in 0..5 {
        ans += dishes[i];
        if i != 4 {
            if dishes[i] % 10 != 0 {
                ans += 10 - dishes[i] % 10;
            }
        }
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"29
20
7
35
120",
            "215"
        ),
        (
            r"101
86
119
108
57",
            "481"
        ),
        (
            r"123
123
123
123
123",
            "643"
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