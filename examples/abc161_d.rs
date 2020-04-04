use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let k: usize = iterator.next().unwrap().parse().unwrap();

    let vec = construct(k);

    return format!("{}", vec[k - 1]);
}

fn construct(k: usize) -> Vec<usize> {
    let mut phase: Vec<usize> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    let mut result: Vec<usize> = Vec::new();
    result.extend(phase.iter());

    while result.len() < k {
        let mut next_phase: Vec<usize> = Vec::new();
        for l in phase {
            let generated = generate(l);
            next_phase.extend(generated);
        }
        result.extend(next_phase.iter());
        phase = next_phase;
    }
    result
}

fn generate(l: usize) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::new();
    let last = l % 10;
    if last == 0 {
        result.push(l * 10 + 0);
        result.push(l * 10 + 1);
    } else if last == 9 {
        result.push(l * 10 + 8);
        result.push(l * 10 + 9);
    } else {
        result.push(l * 10 + last - 1);
        result.push(l * 10 + last);
        result.push(l * 10 + last + 1);
    }
    return result;
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"15",
            "23"
        ),
        (
            r"1",
            "1"
        ),
        (
            r"13",
            "21"
        ),
        (
            r"100000",
            "3234566667"
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