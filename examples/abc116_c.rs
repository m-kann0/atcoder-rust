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
    let mut heights: Vec<usize> = (0..n)
        .map(|_| iterator.next().unwrap().parse().unwrap())
        .collect();

    let mut count: usize = 0;
    loop {
        if heights.iter().all(|x| *x == 0) {
            break;
        }
        let mut grouped = false;
        for i in 0..n {
            if heights[i] == 0 {
                grouped = false;
                continue;
            }

            heights[i] -= 1;
            if !grouped {
                grouped = true;
                count += 1;
            }
        }
    }

    return count.to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4
1 2 2 1",
            "2"
        ),
        (
            r"5
3 1 2 3 1",
            "5"
        ),
        (
            r"8
4 23 75 0 23 96 50 100",
            "221"
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