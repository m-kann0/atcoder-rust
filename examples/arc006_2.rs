use std::io::stdin;

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();

    let mut iterator = buf.split_whitespace();

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let l: usize = iterator.next().unwrap().parse().unwrap();

    let mut positions: Vec<usize> = Vec::new();
    for i in 1..(n + 1) {
        positions.push(i);
    }

    for _ in 0..l {
        let mut buf = String::new();
        stdin().read_line(&mut buf).unwrap();

        let line: Vec<char> = buf.chars().collect();
        for i in 0..(n - 1) {
            let c = line[2 * i + 1];
            if c == '-' {
                positions.swap(i, i + 1);
            }
        }
    }

    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    let line: Vec<char> = buf.chars().collect();
    let position = line.iter().position(|it| *it == 'o').unwrap();
    let ans = positions[position / 2];
    println!("{}", ans);
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 2
| |-|
|-| |
o    ",
            r"3"
        ),
        (
            r"10 2
| |-| |-| |-| |-| |
|-| |-| |-| |-| |-|
            o      ",
            r"9"
        ),
        (
            r"1 5
|
|
|
|
|
o",
            r"1"
        ),
        (
            r"4 2
| | | |
| | | |
      o",
            r"4"
        ),
        (
            r"9 8
| | | | | | | | |
|-| | |-| | |-| |
| | |-| | |-| | |
| |-| | | | | |-|
| | | |-| | | |-|
| | |-| |-| | | |
|-| | |-| | |-| |
| | | | | |-| | |
            o    ",
            r"3"
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