# Template
```rust
use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    
    
    return String::new();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![

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
```

# Bookmarklet
```js
var sections = document.querySelectorAll('.part section');
var ta = document.createElement('textarea');
for (var i = 0; i < sections.length; i++) {
    var section = sections[i];
    if (section.children[0].innerText.startsWith('入力例')) {
        var input = section.children[2].innerText.trim();
        var output = sections[i + 1].children[2].innerText.trim();
        ta.value += '(\nr"' + input + '",\n"' + output + '"\n),\n';
    }
}
ta.value = ta.value.trim();
document.body.appendChild(ta);
ta.select();
document.execCommand('copy');
ta.parentElement.removeChild(ta);
```
