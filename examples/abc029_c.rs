use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: u32 = iterator.next().unwrap().parse().unwrap();

    let mut result = String::new();

    for i in 0..3_u32.pow(n) {
        let mut x = i;
        let mut s = String::new();
        while x != 0 {
            s = format!("{}{}", x % 3, s);
            x /= 3;
        }
        s = format!("{:0>1$}", s, n as usize);
        let p = s.chars()
            .map(|c| {
                if c == '0' {
                    "a".to_string()
                } else if c == '1' {
                    "b".to_string()
                } else {
                    "c".to_string()
                }
            })
            .collect::<Vec<String>>()
            .join("");
        result.push_str(&format!("{}\n", p))
    }

    return result.trim().to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"1",
            "a
b
c"
        ),
        (
            r"2",
            "aa
ab
ac
ba
bb
bc
ca
cb
cc"
        ),
        (
            r"3",
            "aaa
aab
aac
aba
abb
abc
aca
acb
acc
baa
bab
bac
bba
bbb
bbc
bca
bcb
bcc
caa
cab
cac
cba
cbb
cbc
cca
ccb
ccc"
        ),
        (
            r"4",
            "aaaa
aaab
aaac
aaba
aabb
aabc
aaca
aacb
aacc
abaa
abab
abac
abba
abbb
abbc
abca
abcb
abcc
acaa
acab
acac
acba
acbb
acbc
acca
accb
accc
baaa
baab
baac
baba
babb
babc
baca
bacb
bacc
bbaa
bbab
bbac
bbba
bbbb
bbbc
bbca
bbcb
bbcc
bcaa
bcab
bcac
bcba
bcbb
bcbc
bcca
bccb
bccc
caaa
caab
caac
caba
cabb
cabc
caca
cacb
cacc
cbaa
cbab
cbac
cbba
cbbb
cbbc
cbca
cbcb
cbcc
ccaa
ccab
ccac
ccba
ccbb
ccbc
ccca
cccb
cccc"
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