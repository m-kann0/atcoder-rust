#![allow(non_snake_case)]

use std::io::Read;
use std::cmp::{max, min};

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let N: usize = iterator.next().unwrap().parse().unwrap();
    let W: usize = iterator.next().unwrap().parse().unwrap();
    let mut v = Vec::new();
    let mut w = Vec::new();
    let mut v_max = 0;
    let mut w_max = 0;
    for _ in 0..N {
        let vi: usize = iterator.next().unwrap().parse().unwrap();
        let wi: usize = iterator.next().unwrap().parse().unwrap();
        v_max = max(v_max, vi);
        w_max = max(w_max, wi);
        v.push(vi);
        w.push(wi);
    }

    let ans = if w_max <= 1000 {
        solve2(N, W, &v, &w)
    } else if v_max <= 1000 {
        solve3(N, W, &v, &w)
    } else {
        solve1(N, W, &v, &w)
    };
    ans.to_string()
}

fn solve1(N: usize, W: usize, v: &Vec<usize>, w: &Vec<usize>) -> usize {
    // 前半分を全列挙
    let mut ps = Vec::new();
    let n2 = N / 2;
    for i in 0..(1 << n2) {
        let mut sw = 0;
        let mut sv = 0;
        for j in 0..n2 {
            if (i >> j & 1) > 0 {
                sw += w[j];
                sv += v[j];
            }
        }
        ps.push((sw, sv));
    }
    // eprintln!("ps = {:?}", ps);
    // 無駄な要素を取り除く
    ps.sort();
    let mut ps2 = Vec::new();
    ps2.push(ps[0].clone());
    let mut last = ps[0].clone();
    for i in 1..(1 << n2) {
        if ps[i].0 > last.0 {
            if ps[i].1 > last.1 {
                ps2.push(ps[i].clone());
                last = ps[i].clone();
            }
        } else {
            ps2.push(ps[i].clone());
            last = ps[i].clone();
        }
    }
    let ps = ps2;
    // 後ろ半分を全列挙
    let mut result = 0;
    for i in 0..(1 << (N - n2)) {
        let mut sw = 0;
        let mut sv = 0;
        for j in 0..(N - n2) {
            if (i >> j & 1) > 0 {
                sw += w[n2 + j];
                sv += v[n2 + j];
            }
        }
        if sw > W {
            continue;
        }
        // 前半分から二分探索
        let mut ok: isize = 0;
        let mut ng: isize = ps.len() as isize;
        while (ok - ng).abs() > 1 {
            let mid = (ok + ng) / 2;
            if ps[mid as usize].0 + sw <= W {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        let tv = ps[ok as usize].1;
        result = max(result, sv + tv);
    }
    result
}

fn solve2(N: usize, W: usize, v: &Vec<usize>, w: &Vec<usize>) -> usize {
    const N_MAX: usize = 200;
    const W_MAX: usize = 1000;
    let mut dp: Vec<Vec<usize>> = vec![vec![0; N_MAX * W_MAX + 1]; N + 1];
    for i in 0..N {
        for j in 0..=(N_MAX * W_MAX) {
            dp[i + 1][j] = max(dp[i + 1][j], dp[i][j]);
            if j >= w[i] {
                dp[i + 1][j] = max(dp[i + 1][j], dp[i][j - w[i]] + v[i]);
            }
        }
    }
    let mut ans = 0;
    for j in 0..=min(W, N_MAX * W_MAX) {
        ans = max(ans, dp[N][j]);
    }
    ans
}

fn solve3(N: usize, W: usize, v: &Vec<usize>, w: &Vec<usize>) -> usize {
    const N_MAX: usize = 200;
    const V_MAX: usize = 1000;
    const INF: usize = 1_000_000_000_000;
    let mut dp = vec![vec![INF; N_MAX * V_MAX + 1]; N_MAX + 1];
    dp[0][0] = 0;
    for i in 0..N {
        for j in 0..=(N_MAX * V_MAX) {
            dp[i + 1][j] = min(dp[i + 1][j], dp[i][j]);
            if j >= v[i] {
                dp[i + 1][j] = min(dp[i + 1][j], dp[i][j - v[i]] + w[i]);
            }
        }
    }
    let mut ans = 0;
    for j in 0..=(N_MAX * V_MAX) {
        if dp[N][j] <= W {
            ans = j;
        }
    }
    ans
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 10
15 9
10 6
6 4",
            "16"
        ),
        (
            r"30 499887702
128990795 137274936
575374246 989051853
471048785 85168425
640066776 856699603
819841327 611065509
704171581 22345022
536108301 678298936
119980848 616908153
117241527 28801762
325850062 478675378
623319578 706900574
998395208 738510039
475707585 135746508
863910036 599020879
340559411 738084616
122579234 545330137
696368935 86797589
665665204 592749599
958833732 401229830
371084424 523386474
463433600 5310725
210508742 907821957
685281136 565237085
619500108 730556272
88215377 310581512
558193168 136966252
475268130 132739489
303022740 12425915
122379996 137199296
304092766 23505143",
            "3673016420"
        ),
        (
            r"10 2921
981421680 325
515936168 845
17309336 371
788067075 112
104855562 96
494541604 960
32007355 161
772339969 581
55112800 248
98577050 22",
            "3657162058"
        ),
        (
            r"10 936447862
854 810169801
691 957981784
294 687140254
333 932608409
832 42367415
642 727293784
139 870916042
101 685539955
853 243593312
369 977358410",
            "1686"
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