use std::io::stdin;

fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();

    let mut words: Vec<String> = line.split_whitespace()
        .map(|s| s.to_string())
        .collect();

    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    let n: usize = line.trim().parse().unwrap();

    let mut ng_words = Vec::with_capacity(n);
    for _ in 0..n {
        let mut line = String::new();
        stdin().read_line(&mut line).unwrap();
        ng_words.push(line.trim().to_string());
    }

    for i in 0..words.len() {
        if is_ng(&words[i], &ng_words) {
            words[i] = words[i].replace(|_c| true, "*");
        }
    }

    let ans = words.join(" ");
    println!("{}", ans);
}

fn is_ng(word: &str, ng_words: &Vec<String>) -> bool {
    for ng_word in ng_words {
        if word.len() != ng_word.len() {
            continue;
        }
        let mut is_same = true;
        for (w, n) in word.chars().zip(ng_word.chars()) {
            if n == '*' {
                continue;
            }
            if w == n {
                continue;
            }
            is_same = false;
            break;
        }
        if is_same {
            return true;
        }
    }
    false
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"abc aaa ababa abcba abc
2
abc
**a**",
            r"*** aaa ***** abcba ***"
        ),
        (
            r"aaaa aaa aaaaaa aaaa
3
a
aa
aaa",
            r"aaaa *** aaaaaa aaaa"
        ),
        (
            r"i have a pen
1
*",
            r"* have * pen"
        ),
        (
            r"digital arts
1
digital*arts",
            r"digital arts"
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