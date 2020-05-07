use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let h: usize = iterator.next().unwrap().parse().unwrap();
    let w: usize = iterator.next().unwrap().parse().unwrap();

    let n: usize = iterator.next().unwrap().parse().unwrap();

    let mut a: Vec<(usize, usize)> = Vec::new();
    for i in 0..n {
        let ai: usize = iterator.next().unwrap().parse().unwrap();
        a.push((i + 1, ai));
    }

    let mut result: Vec<Vec<usize>> = vec![vec![0; w]; h];

    let mut color = 0;
    let mut ai = 0;
    for i in 0..h {
        for j in 0..w {
            if ai == 0 {
                let pair = a.pop().unwrap();
                color = pair.0;
                ai = pair.1;
            }
            if i % 2 == 0 {
                result[i][j] = color;
            } else {
                result[i][w - j - 1] = color;
            }
            ai -= 1;
        }
    }

    let mut output = String::new();
    for i in 0..h {
        let mut line = String::new();
        for j in 0..w {
            line.push_str(&format!("{} ", result[i][j]))
        }
        output.push_str(&format!("{}\n", line.trim()))
    }
    return output.trim().to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2 2
3
2 1 1",
            "1 1
2 3"
        ),
        (
            r"3 5
5
1 2 3 4 5",
            "1 4 4 4 3
2 5 4 5 3
2 5 5 5 3"
        ),
        (
            r"1 1
1
1",
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