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
    let mut playlist: Vec<(String, usize)> = Vec::new();
    for _ in 0..n {
        let s = iterator.next().unwrap().to_string();
        let t: usize = iterator.next().unwrap().parse().unwrap();
        playlist.push((s, t));
    }
    let x: String = iterator.next().unwrap().to_string();

    let mut ans: usize = 0;
    let mut sleeping: bool = false;
    for music in playlist {
        if sleeping {
            ans += music.1;
        } else {
            if music.0 == x {
                sleeping = true;
            }
        }
    }
    return ans.to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3
dwango 2
sixth 5
prelims 25
dwango",
            "30"
        ),
        (
            r"1
abcde 1000
abcde",
            "0"
        ),
        (
            r"15
ypnxn 279
kgjgwx 464
qquhuwq 327
rxing 549
pmuduhznoaqu 832
dagktgdarveusju 595
wunfagppcoi 200
dhavrncwfw 720
jpcmigg 658
wrczqxycivdqn 639
mcmkkbnjfeod 992
htqvkgkbhtytsz 130
twflegsjz 467
dswxxrxuzzfhkp 989
szfwtzfpnscgue 958
pmuduhznoaqu",
            "6348"
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