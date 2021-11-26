#[cfg(test)]
#[rstest::rstest]
#[case(vec![1,2,3,4,5], Some(12))]
#[case(vec![2,3,4,5, 10], Some(12))]
#[case(vec![4,5, 10, 20], Some(0))]
fn ant_1_6_triangle_test_rstest(#[case] mut lengths: Vec<u8>, #[case] ans: Option<u8>) {
    let ret = ant_1_6_triangle(&mut lengths);
    assert_eq!(ret, ans);
}
#[cfg(test)]
proptest::proptest! {
    #[test]
    fn ant_1_6_triangle_test_prop(mut lengths: Vec<u8>){
        let _ = ant_1_6_triangle(&mut lengths);
    }
}
pub fn ant_1_6_triangle(lengths: &mut [u8]) -> Option<u8> {
    lengths.sort_unstable();
    lengths.reverse();
    for (i, length) in lengths.iter().enumerate() {
        let a = lengths.get(i + 1);
        let b = lengths.get(i + 2);
        match (a, b) {
            (Some(a), Some(b)) => {
                if let Some(c) = a.checked_add(*b) {
                    if c > *length {
                        return c.checked_add(*length);
                    }
                }
            }
            _ => {}
        }
    }
    Some(0)
}

#[cfg(test)]
#[rstest::rstest]
#[case(10, vec![2,6,7], Some((4, 8)))]
#[case(10, vec![1,3,5,7,9], Some((5, 9)))]
fn ant_1_6_ants_test_rstest(#[case] l: u8, #[case] mut x: Vec<u8>, #[case] ans: Option<(u8, u8)>) {
    let ret = ant_1_6_ants(l, &mut x);
    assert_eq!(ret, ans)
}
#[cfg(test)]
proptest::proptest! {
    #[test]
    fn ant_1_6_ants_test_prop(l: u8, mut x: Vec<u8>){
        let _ = ant_1_6_ants(l, &mut x);
    }
}
pub fn ant_1_6_ants(length: u8, ants_pos_from_left: &[u8]) -> Option<(u8, u8)> {
    if ants_pos_from_left.is_empty() {
        return None;
    }
    let max = {
        let r = ants_pos_from_left.iter().copied().max().unwrap();
        let l = ants_pos_from_left.iter().copied().min().unwrap();
        if let Some(tmp) = length.checked_sub(r) {
            if tmp < l {
                r
            } else {
                length - l
            }
        } else {
            return None;
        }
    };
    let min = {
        let harflen = length / 2;
        let (leftpart, rightpart): (Vec<u8>, Vec<u8>) = ants_pos_from_left
            .iter()
            .copied()
            .partition(|a| *a < harflen);
        let rightpart = rightpart
            .into_iter()
            .map(|o| length - o)
            .collect::<Vec<_>>();
        let lmax = leftpart.iter().max();
        let rmax = rightpart.iter().max();
        if let (Some(lmax), Some(rmax)) = (lmax, rmax) {
            let max = [*lmax, *rmax].iter().copied().max().unwrap();
            max
        } else {
            return None;
        }
    };
    Some((min, max))
}
#[cfg(test)]
#[rstest::rstest]
//#[case(10, vec![1,3,5], Some(true))]
//#[case(9, vec![1,3,5], Some(false))]
//#[case(10, vec![1,3,5,7], Some(true))]
//#[case(31, vec![1,3,5,7,11], Some(false))]
//#[case(21, vec![1,3,5,7,11], Some(false))]
//#[case(22, vec![1,3,5,7,11], Some(true))]
//#[case(4, vec![1], Some(true))]
//#[case(5, vec![1,2], Some(true))]
//#[case(187, vec![25, 0, 117, 46, 20], Some(true))]
#[case(96, vec![28, 20, 29], Some(true))]
fn ant_1_1_kuji_test_rstest(#[case] m: u8, #[case] mut k: Vec<u8>, #[case] ans: Option<bool>) {
    let actual = ant_1_1_kuji_primitive(m, &mut k);
    let ret = ant_1_1_kuji(m, &mut k);
    assert_eq!(ret, actual);
    assert_eq!(ret, ans);
    assert_eq!(actual, ans);
}
#[cfg(test)]
proptest::proptest! {
    #[test]
    fn ant_1_1_kuji_test_prop(m: u8, mut k: Vec<u8>){
        let actual = ant_1_1_kuji_primitive(m, &mut k);
        let ret = ant_1_1_kuji(m, &mut k);
        assert_eq!(ret, actual);
    }
}
pub fn ant_1_1_kuji_primitive(m: u8, k: &mut [u8]) -> Option<bool> {
    use itertools::Itertools;
    let mut actual = None;
    if m == 0 {
        actual = Some(false);
    }
    if k.is_empty() {
        actual = Some(false);
    }
    let mut item_count = 0_u64;
    for (((a, b), c), d) in k
        .iter()
        .cartesian_product(k.iter())
        .cartesian_product(k.iter())
        .cartesian_product(k.iter())
    {
        item_count += 1;
        let checked_sum = vec![a, b, c, d]
            .iter()
            .copied()
            .fold(Some(0_u8), |sum, o| sum.and_then(|sum| sum.checked_add(*o)));
        match checked_sum {
            Some(o) if o == m => {
                actual = Some(true);
                dbg!((a, b, c, d, a + b + c + d, m));
                break;
            }
            Some(_) => {
                actual = Some(false);
            }
            None => {
                continue;
            }
        }
    }
    if item_count == 0 {
        actual = Some(false);
    }
    actual
}
pub fn ant_1_1_kuji(m: u8, k: &mut [u8]) -> Option<bool> {
    k.sort_unstable();
    dbg!(&k);
    for fst in k.iter().copied().rev() {
        if m <= fst {
            continue;
        }
        if fst.checked_add(2).is_none() {
            continue;
        }
        if m <= fst + 2 {
            continue;
        }
        dbg!(fst);
        'a: for snd in k.iter().copied().rev() {
            dbg!((fst, snd));
            let trd_acceptable = m - (fst + snd) - 1;
            dbg!(trd_acceptable);
            let trd = loop {
                match k.binary_search(&trd_acceptable) {
                    Ok(i) => k.get(i).unwrap(),
                    Err(i) if i > 0 => match dbg!(k.get(i - 1)) {
                        Some(o) if o <= &trd_acceptable => {
                            break o;
                        }
                        _ => {
                            continue 'a;
                        }
                    },
                    _ => {
                        continue 'a;
                    }
                };
            };
            let fth = m - (trd + snd + fst);
            match k.binary_search(&fth) {
                Ok(..) => {
                    return Some(true);
                }
                Err(..) => {}
            }
        }
    }
    Some(false)
}

#[cfg(test)]
#[rstest::rstest]
#[case(vec![4,3,1,1,2,10,2], 6, Some(4))]
#[case(vec![10,10,10,10,0,10],10,Some(6))]
#[case(vec![10,10,10,10,10,10],9, Some(0))]
#[case(vec![1,2,3,4],0, Some(0))]
fn abc032_c_test_rstest(#[case] s: Vec<u64>, #[case] k: u64, #[case] ans: Option<u64>) {
    let ret = abc032_c_primitive(&s, k);
    assert_eq!(ret, ans);
    let ret = abc032_c(&s, k);
    assert_eq!(ret, ans);
}
#[cfg(test)]
proptest::proptest! {
    #[test]
    fn abc032_c_test_prop(mut s: Vec<u8>, k:u64){
        let s = s.into_iter().map(|o| o as u64).collect::<Vec<_>>();
        let a = abc032_c(&s, k);
        let b = abc032_c_primitive(&s, k);
      assert_eq!(a, b);
    }
}
// https://atcoder.jp/contests/abc032/tasks/abc032_c
pub fn abc032_c(s: &[u64], k: u64) -> Option<u64> {
    let n = s.len();
    let has_zero = s.contains(&0);
    if has_zero {
        return Some(n as u64);
    }
    let mut ans_lens: Vec<u64> = vec![];
    for left_i in 0..s.len() {
        let mut prd: u64 = 1;
        let mut len: u64 = 0;
        for right_i in left_i..s.len() {
            let prd_ = if let Some(prd_) = prd.checked_mul(*s.get(right_i).unwrap()) {
                prd_
            } else {
                println!("ha??{}:{}:{}", len, prd, right_i);
                break;
            };
            // ans_len を更新すべき条件
            if prd_ <= k {
                // 区間の累乗が k を超えてない場合
                // len 更新 (+1 は インデックス 差分が0のときは 1 要素と考えるので)
                len = (right_i - left_i + 1) as u64;
            }
            prd = prd_;
        }
        ans_lens.push(len);
    }
    println!("{:?}", ans_lens);
    let ans = ans_lens.into_iter().max().unwrap_or(0);
    Some(ans)
}
pub fn abc032_c_primitive(s: &[u64], k: u64) -> Option<u64> {
    let n = s.len();
    let has_zero = s.contains(&0);
    if has_zero {
        return Some(n as u64);
    }
    let mut right_i = 0;
    let mut prd: u64 = 1;
    let mut ans = 0;
    for left_i in 0..n {
        loop {
            if right_i < n {
                let prd_ = if let Some(prd_) = prd.checked_mul(*s.get(right_i).unwrap()) {
                    prd_
                } else {
                    break;
                };
                if prd_ <= k {
                    prd = prd_;
                    right_i += 1;
                    continue;
                }
            }
            break;
        }
        ans = ans.max((right_i - left_i) as u64);
        if left_i == right_i {
            right_i += 1;
        } else {
            prd /= s.get(left_i).unwrap();
        }
    }
    Some(ans as u64)
}
