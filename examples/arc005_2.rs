use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let x: usize = iterator.next().unwrap().parse().unwrap();
    let y: usize = iterator.next().unwrap().parse().unwrap();
    let w = iterator.next().unwrap();
    let mut table: Vec<Vec<char>> = Vec::new();
    for _ in 0..9 {
        table.push(iterator.next().unwrap().chars().collect());
    }

    let mut dx: isize = 0;
    let mut dy: isize = 0;
    if w.contains('R') {
        dx = 1;
    } else if w.contains('L') {
        dx = -1;
    }
    if w.contains('U') {
        dy = -1;
    } else if w.contains('D') {
        dy = 1;
    }

    let mut x: isize = x as isize - 1;
    let mut y: isize = y as isize - 1;

    let mut result = String::new();
    for _ in 0..4 {
        result.push(table[y as usize][x as usize]);
        if x + dx < 0 || x + dx >= 9 {
            dx = -dx;
        }
        if y + dy < 0 || y + dy >= 9 {
            dy = -dy;
        }
        x += dx;
        y += dy;
    }
    return result;
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 5 R
790319030
091076399
143245946
590051196
398226115
442567154
112705290
716433235
221041645",
            r"8226"
        ),
        (
            r"8 9 LU
206932999
471100777
973172688
108989704
246954192
399039569
944715218
003664867
219006823",
            r"2853"
        ),
        (
            r"5 7 D
271573743
915078603
102553534
996473623
595593497
573572507
340348994
253066837
643845096",
            r"4646"
        ),
        (
            r"2 2 LU
729142134
509607882
640003027
215270061
214055727
745319402
777708131
018697986
277156993",
            r"0700"
        ),
        (
            r"8 7 RD
985877833
469488482
218647263
856777094
012249580
845463670
919136580
011130808
874387671",
            r"8878"
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