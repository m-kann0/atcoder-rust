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
    let mut rates: Vec<usize> = Vec::new();
    let mut hands: Vec<usize> = Vec::new();
    let mut counts: Vec<Vec<usize>> = vec![vec![0; 3]; 100_001];
    for _ in 0..n {
        let rate: usize = iterator.next().unwrap().parse().unwrap();
        let hand: usize = iterator.next().unwrap().parse().unwrap();
        rates.push(rate);
        hands.push(hand - 1);
        counts[rate][hand - 1] += 1;
    }

    let mut sum: Vec<usize> = vec![0; 100_001];
    for i in 1..100_001 {
        for j in 0..3 {
            sum[i] += counts[i][j];
        }
        sum[i] += sum[i - 1];
    }

    let mut result = String::new();
    for i in 0..n {
        let rate = rates[i];
        let hand = hands[i];

        let mut win: usize = 0;
        let mut lose: usize = 0;
        let mut even: usize = 0;

        win += sum[rate - 1];
        lose += sum[100_000] - sum[rate];

        if hand == 0 {
            win += counts[rate][1];
            lose += counts[rate][2];
            even += counts[rate][0] - 1;
        } else if hand == 1 {
            win += counts[rate][2];
            lose += counts[rate][0];
            even += counts[rate][1] - 1;
        } else {
            win += counts[rate][0];
            lose += counts[rate][1];
            even += counts[rate][2] - 1;
        }

        result.push_str(&format!("{} {} {}\n", win, lose, even));
    }
    result.trim().to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"6
2 1
2 2
3 2
5 3
2 2
3 3",
            "2 3 0
0 4 1
4 1 0
5 0 0
0 4 1
3 2 0"
        ),
        (
            r"2
1999 3
2000 1",
            "0 1 0
1 0 0"
        ),
        (
            r"8
3200 2
2800 3
2800 2
2700 1
2800 2
3200 1
2700 1
3200 3",
            "6 1 0
2 5 0
3 3 1
0 6 1
3 3 1
6 1 0
0 6 1
6 1 0"
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