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

    let mut ans: Vec<usize> = Vec::new();

    let mut sum = 0;
    let mut i = 1;
    while sum < n {
        sum += i;
        ans.push(i);
        i += 1;
    }

    let diff = sum - n;
    if diff > 0 {
        let position = ans.iter().position(|&x| x == diff).unwrap();
        ans.remove(position);
    }

    return ans.iter()
        .map(|x| x.to_string() + "\n")
        .collect::<String>()
        .trim()
        .to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4",
            "1
3"
        ),
        (
            r"7",
            "1
2
4"
        ),
        (
            r"1",
            "1"
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