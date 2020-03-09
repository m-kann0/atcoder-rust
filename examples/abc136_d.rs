use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let s = iterator.next().unwrap().chars().collect::<Vec<char>>();

    let mut part = Vec::new();

    let mut ans: Vec<usize> = Vec::with_capacity(s.len());

    let n = s.len();
    for i in 0..n {
        part.push(s[i]);
        if i >= n - 1 || (s[i] == 'L' && s[i + 1] == 'R') {
            let mut a = calc(&part);
            ans.append(&mut a);
            part = Vec::new();
        }
    }

    return ans.iter().map(|x| format!("{} ", x)).collect::<String>().trim().to_string();
}

fn calc(part: &Vec<char>) -> Vec<usize> {
    let l_position = part.iter().position(|&ch| ch == 'L').unwrap();
    let l_count = part.len() - l_position;
    let r_count = part.len() - l_count;

    let mut result = vec![0; part.len()];
    result[l_position - 1] = (r_count + 1) / 2 + l_count / 2;
    result[l_position] = r_count / 2 + (l_count + 1) / 2;
    result
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"RRLRL",
            "0 1 2 1 1"
        ),
        (
            r"RRLLLLRLRRLL",
            "0 3 3 0 0 0 1 1 0 2 2 0"
        ),
        (
            r"RRRLLRLLRRRLLLLL",
            "0 0 3 2 0 2 1 0 0 0 4 4 0 0 0 0"
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