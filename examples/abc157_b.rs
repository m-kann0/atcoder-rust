use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let mut vec: Vec<Vec<usize>> = Vec::new();
    for _i in 0..3 {
        let mut v = Vec::new();
        for _j in 0..3 {
            v.push(iterator.next().unwrap().parse().unwrap());
        }
        vec.push(v);
    }

    let n: usize = iterator.next().unwrap().parse().unwrap();
    for _i in 0..n {
        let b: usize = iterator.next().unwrap().parse().unwrap();
        for j in 0..3 {
            for k in 0..3 {
                if vec[j][k] == b {
                    vec[j][k] =0;
                }
            }
        }
    }

    return if (vec[0][0] == 0 && vec[0][1] == 0 && vec[0][2] == 0)
        || (vec[1][0] == 0 && vec[1][1] == 0 && vec[1][2] == 0)
        || (vec[2][0] == 0 && vec[2][1] == 0 && vec[2][2] == 0)
        || (vec[0][0] == 0 && vec[1][0] == 0 && vec[2][0] == 0)
        || (vec[0][1] == 0 && vec[1][1] == 0 && vec[2][1] == 0)
        || (vec[0][2] == 0 && vec[1][2] == 0 && vec[2][2] == 0)
        || (vec[0][0] == 0 && vec[1][1] == 0 && vec[2][2] == 0)
        || (vec[0][2] == 0 && vec[1][1] == 0 && vec[2][0] == 0) {
        "Yes".to_string()
    } else {
        "No".to_string()
    }
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"84 97 66
79 89 11
61 59 7
7
89
7
87
79
24
84
30",
            "Yes"
        ),
        (
            r"41 7 46
26 89 2
78 92 8
5
6
45
16
57
17",
            "No"
        ),
        (
            r"60 88 34
92 41 43
65 73 48
10
60
43
88
11
48
73
65
41
92
34",
            "Yes"
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