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
    let k: usize = iterator.next().unwrap().parse().unwrap();
    let a: Vec<usize> = (0..n)
        .map(|_| iterator.next().unwrap().parse().unwrap())
        .collect();

    if k <= n {
        let mut current: usize = 0;
        let mut count: usize = 0;
        while count < k {
            current = a[current] - 1;
            count += 1;
        }

        return (current + 1).to_string();
    }

    let mut current: usize = 0;
    let mut visited: Vec<bool> = vec![false; n];
    let mut route: Vec<usize> = Vec::new();
    loop {
        if visited[current] {
            break;
        }
        route.push(current);
        visited[current] = true;
        current = a[current] - 1;
    }

    let position = route.iter().position(|it| *it == current).unwrap();
    let m = (k - position) % (route.len() - position);
    let ans = route[position + m];
    return (ans + 1).to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4 5
3 2 4 1",
            "4"
        ),
        (
            r"6 727202214173249351
6 5 2 5 3 2",
            "2"
        ),
        (
            r"4 3
3 2 4 1",
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