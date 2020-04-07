use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let vec = vec!['a', 't', 'c', 'o', 'd', 'e', 'r'];

    let s = iterator.next().unwrap();
    let t = iterator.next().unwrap();

    let mut can_win = true;

    for (c1, c2) in s.chars().zip(t.chars()) {
        if c1 == c2 {
            continue;
        }
        if c1 == '@' && vec.contains(&c2) {
            continue;
        }
        if c2 == '@' && vec.contains(&c1) {
            continue;
        }
        can_win = false;
        break;
    }

    return if can_win {
        "You can win".to_string()
    } else {
        "You will lose".to_string()
    }
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"ch@ku@ai
choku@@i",
            r"You can win"
        ),
        (
            r"aoki
@ok@",
            r"You will lose"
        ),
        (
            r"arc
abc",
            r"You will lose"
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
