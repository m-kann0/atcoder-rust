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
    let x: usize = iterator.next().unwrap().parse().unwrap();
    let m: usize = iterator.next().unwrap().parse().unwrap();

    let mut sum: usize = 0;
    let mut history: Vec<usize> = Vec::new();
    let mut positions: Vec<usize> = vec![0; m];
    let mut current = x;
    sum += current;
    history.push(current);
    positions[current] = 1;
    let mut count: usize = 2;
    while count <= n {
        current = (current * current) % m;
        if positions[current] != 0 {
            let loops = count - positions[current];
            let rest = n - count + 1;
            let mut sum_of_loop = 0;
            for i in (positions[current] - 1)..(count - 1) {
                sum_of_loop += history[i];
            }
            sum += sum_of_loop * (rest / loops);
            for i in (positions[current] - 1)..(positions[current] - 1 + rest % loops) {
                sum += history[i];
            }
            break;
        }
        sum += current;
        history.push(current);
        positions[current] = count;
        count += 1;
    }
    sum.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"6 2 1001",
            "1369"
        ),
        (
            r"1000 2 16",
            "6"
        ),
        (
            r"10000000000 10 99959",
            "492443256176507"
        ),
        (
            r"5 2 11",
            "23"
        ),
        (
            r"6 2 11",
            "27"
        ),
        (
            r"7 2 11",
            "32"
        ),
        (
            r"8 2 11",
            "35"
        ),
        (
            r"9 2 11",
            "44"
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