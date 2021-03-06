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
    let mut total = 0;
    for _ in 0..h {
        total += iterator.next().unwrap().chars()
            .filter(|c| *c == '#')
            .count();
    }

    return if total == h + w - 1 {
        "Possible".to_string()
    } else {
        "Impossible".to_string()
    }
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4 5
##...
.##..
..##.
...##",
            "Possible"
        ),
        (
            r"5 3
###
..#
###
#..
###",
            "Impossible"
        ),
        (
            r"4 5
##...
.###.
.###.
...##",
            "Impossible"
        ),
        (
            r"4 5
#####
....#
....#
...##",
            "Impossible"
        ),
        (
            r"4 5
#....
#....
#...#
#####",
            "Impossible"
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