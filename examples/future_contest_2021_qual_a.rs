use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let mut cards: Vec<(usize, usize)> = Vec::new();
    for _ in 0..100 {
        let x: usize = iterator.next().unwrap().parse().unwrap();
        let y: usize = iterator.next().unwrap().parse().unwrap();
        cards.push((x, y));
    }

    let mut result: Vec<char> = Vec::new();
    let mut current = (0, 0);
    for i in 0..100 {
        let card = cards[i];
        if card.0 > current.0 {
            for _ in 0..(card.0 - current.0) {
                result.push('D');
            }
        } else if card.0 < current.0 {
            for _ in 0..(current.0 - card.0) {
                result.push('U');
            }
        }
        if card.1 > current.1 {
            for _ in 0..(card.1 - current.1) {
                result.push('R');
            }
        } else if card.1 < current.1 {
            for _ in 0..(current.1 - card.1) {
                result.push('L');
            }
        }
        result.push('I');
        current = (card.0, card.1);
    }
    result.iter().collect::<String>()
}

fn evaluate(output: String) -> isize {
    let mut score = 4000;
    let operations: Vec<char> = output.chars().collect();
    for i in 0..operations.len() {
        match operations[i] {
            'U' | 'D' | 'L' | 'R' => {
                score -= 1;
            },
            _ => {},
        }
    }
    score
}

#[test]
fn test_sample() {
    let input = r"15 19
8 1
3 13
2 19
17 10
14 3
3 2
19 4
6 2
18 1
4 4
3 10
0 15
2 5
10 7
6 3
19 12
1 0
19 3
4 1
0 6
10 18
12 12
8 13
6 4
10 2
6 12
2 0
0 11
6 9
8 3
13 9
9 0
11 17
9 4
12 1
1 18
19 19
9 9
2 11
8 19
18 3
2 15
8 16
16 2
4 5
14 4
9 3
15 13
3 0
8 11
15 4
0 7
12 19
18 7
12 17
8 2
6 0
1 7
17 0
16 10
1 6
10 5
4 14
12 7
6 11
6 6
14 19
15 15
17 16
1 12
7 9
2 7
4 7
1 4
5 3
0 17
17 3
5 13
9 16
12 13
11 12
15 18
0 10
19 6
12 4
10 8
15 1
10 13
17 15
1 5
15 14
19 5
17 7
2 18
5 7
12 6
3 6
19 7
1 16";
    println!("{}", evaluate(solve(input)));
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"15 19
8 1
3 13
2 19
17 10
14 3
3 2
19 4
6 2
18 1
4 4
3 10
0 15
2 5
10 7
6 3
19 12
1 0
19 3
4 1
0 6
10 18
12 12
8 13
6 4
10 2
6 12
2 0
0 11
6 9
8 3
13 9
9 0
11 17
9 4
12 1
1 18
19 19
9 9
2 11
8 19
18 3
2 15
8 16
16 2
4 5
14 4
9 3
15 13
3 0
8 11
15 4
0 7
12 19
18 7
12 17
8 2
6 0
1 7
17 0
16 10
1 6
10 5
4 14
12 7
6 11
6 6
14 19
15 15
17 16
1 12
7 9
2 7
4 7
1 4
5 3
0 17
17 3
5 13
9 16
12 13
11 12
15 18
0 10
19 6
12 4
10 8
15 1
10 13
17 15
1 5
15 14
19 5
17 7
2 18
5 7
12 6
3 6
19 7
1 16",
            "DDDDDDDDDDDDDDDRRRRRRRRRRRRRRRRRRRIUUUUUUULLLLLLLLLLLLLLLLLLIUUUUURRRRRRRRRRRRIURRRRRRIDDDDDDDDDDDDDDDLLLLLLLLLIUUULLLLLLLIUUUUUUUUUUULIDDDDDDDDDDDDDDDDRRIUUUUUUUUUUUUULLIDDDDDDDDDDDDLIUUUUUUUUUUUUUURRRIURRRRRRIUUURRRRRIDDLLLLLLLLLLIDDDDDDDDRRIUUUULLLLIDDDDDDDDDDDDDRRRRRRRRRIUUUUUUUUUUUUUUUUUULLLLLLLLLLLLIDDDDDDDDDDDDDDDDDDRRRIUUUUUUUUUUUUUUULLIUUUURRRRRIDDDDDDDDDDRRRRRRRRRRRRIDDLLLLLLIUUUURIUULLLLLLLLLIDDDDLLIUUUURRRRRRRRRRIUUUULLLLLLLLLLLLIUURRRRRRRRRRRIDDDDDDLLIDDLLLLLLIDDDDDRRRRRRIUUUULLLLLLLLLIDDRRRRRRRRRRRRRRRRRIUULLLLLLLLLLLLLIDDDLLLIUUUUUUUUUUURRRRRRRRRRRRRRRRRIDDDDDDDDDDDDDDDDDDRIUUUUUUUUUULLLLLLLLLLIUUUUUUURRIDDDDDDRRRRRRRRIDDDDDDDDDDLLLLLLLLLLLLLLLLIUUUUUUUUUUUUUUUURRRRRRRRRRRRIDDDDDDRIDDDDDDDDLLLLLLLLLLLLLLIUUUUUUUUUUUURRRIDDDDDDDDDDLIUUUUULIDDDDDDRRRRRRRRRRIUUUUUUUUUUUULLLLLLLLLLLLLIDDDDDRRRRRRRRRRRIDDDDDDDLLLLLLLIUUUUUUUUUUUUUUURRRIDDDDDDDDDDDDRRRRRRRRRRRRIDDDDDDLLLLLLLLLLLLIUUUUUURRRRRRRRRRIUUUULLLLLLLLLLLLLLLIUULLIUUUUURRRRRRRIDDDDDDDDDDDDDDDDLLLLLLLIURRRRRRRRRRIUUUUUUUUUUUUUUULLLLIDDDDDDDDDLIUUUUUURRRRRRRRRIDDDDDDDDLLLLLLLIUUUUUURRRRILLLLLIDDDDDDDDRRRRRRRRRRRRRIDLLLLIDDRIUUUUUUUUUUUUUUUULLLLIDDDDDDLLLIUUUUULLIDDIUUULLLIDDDDLIUUUUURRRRRRRRRRRRRRIDDDDDDDDDDDDDDDDDLLLLLLLLLLLLLLIUUUUUUUUUUUURRRRRRRRRRIDDDDRRRIDDDLLLIULIDDDDRRRRRRIUUUUUUUUUUUUUUULLLLLLLLIDDDDDDDDDDDDDDDDDDDLLLLIUUUUUUULLIUURRRRIDDDDDLLLLLLLIUUUUURRRRRRRRRRRRIDDDDDDDRRIUUUUUUUUUUUUUUUULLLLLLLLLLIDDDDDDDDDDDDDDRRRRRRRRRIDDDDLLLLLLLLLIUURRIUUUUUUUUUUUUUUURRRRRRRRRRRIDDDLLLLLLLLLLLIDDDDDDDLIUUUUUUUUUIDDDDDDDDDDDDDDDDRIUUUUUUUUUUUUUUUUUURRRRRRRRRI"
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