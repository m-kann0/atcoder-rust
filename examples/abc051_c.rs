use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let sx: isize = iterator.next().unwrap().parse().unwrap();
    let sy: isize = iterator.next().unwrap().parse().unwrap();
    let tx: isize = iterator.next().unwrap().parse().unwrap();
    let ty: isize = iterator.next().unwrap().parse().unwrap();

    let dx: isize = tx - sx;
    let dy: isize = ty - sy;

    let mut result = String::new();

    for _ in 0..dx {
        result.push('R');
    }
    for _ in 0..dy {
        result.push('U');
    }

    for _ in 0..dx {
        result.push('L');
    }
    for _ in 0..dy {
        result.push('D');
    }

    result.push('D');
    for _ in 0..(dx + 1) {
        result.push('R');
    }
    for _ in 0..(dy + 1) {
        result.push('U');
    }
    result.push('L');

    result.push('U');
    for _ in 0..(dx + 1) {
        result.push('L');
    }
    for _ in 0..(dy + 1) {
        result.push('D');
    }
    result.push('R');

    return result;
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"0 0 1 2",
            "UURDDLLUUURRDRDDDLLU"
        ),
        (
            r"-2 -2 1 1",
            "UURRURRDDDLLDLLULUUURRURRDDDLLDL"
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