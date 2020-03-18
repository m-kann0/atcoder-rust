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

    let mut vec: Vec<Vec<char>> = vec![Vec::new(); 9];
    for _ in 0..n {
        let line = iterator.next().unwrap();
        for (i, c) in line.chars().enumerate() {
            vec[i].push(c);
        }
    }

    let mut ans = 0;
    for i in 0..9 {
        let mut prev = '.';
        for &c in &vec[i] {
            if c == 'x' {
                ans += 1;
            } else if c == 'o' && prev != 'o' {
                ans += 1;
            }
            prev = c;
        }
    }

    return ans.to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"15
.........
.x.......
.........
...x.....
.........
.......o.
.......o.
.......o.
.........
..x.....o
........o
........o
....x...o
.x......o
........o",
            r"7"
        ),
        (
            r"6
..o..x.o.
..o..x.o.
..x..o.o.
..o..o.o.
..o..x.o.
..o..x.o.",
            r"9"
        ),
        (
            r"2
.........
.........",
            r"0"
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