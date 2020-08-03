use std::collections::HashMap;
fn main() {
    let valid_mails = vec![r##"a b c"##, r##"d e f"##];
    let invalid_mails = vec![/*r##"u v w"##,*/ r##"x y z"##];
    let test_mails = vec![
        r##"x y z x x y u v w x"##,
        r##"a c b c x c y c b e b z"##,
        r##"x a x"##,
        r##"a"##,
        r##"a b c a b c"##,
        r##"x a y b z c"##,
        r##"x y z x y z"##,
        r##"x y z a b c a b c x y z"##,
        r##"x a c b c x c y c b e b z z y z y z y z y z x"##,
    ];
    let valid_mails_num = valid_mails.len();
    let invalid_mails_num = invalid_mails.len();
    let all_mails_num = valid_mails_num + invalid_mails_num;
    let p_valid = valid_mails_num as f64 / all_mails_num as f64;
    let p_invalid = invalid_mails_num as f64 / all_mails_num as f64;
    // let p_omega = p_valid + p_invalid;
    let p_f_valid = normalize(&count(&valid_mails));
    let p_f_invalid = normalize(&count(&invalid_mails));
    // let p_f_all = normalize(&count(&[valid_mails, invalid_mails].concat()));
    // ここまでが学習
    for test_mail in test_mails {
        let tokens = {
            let mut tokens = test_mail.split(" ").collect::<Vec<_>>();
            tokens.sort();
            dbg!(tokens)
        };
        // dbg!(&count(&[test_mail]), tokens);
        let oo = tokens.iter().fold(1_f64, |a, k| {
            let valid = p_f_valid.get(*k).unwrap_or(&0.0000000001_f64) + 1_f64;
            let invalid = p_f_invalid.get(*k).unwrap_or(&0.0000000001_f64) + 1_f64;
            // dbg!((k, valid, invalid, (invalid / valid).ln(), a));
            a + (invalid / valid).ln()
        });
        let is_spam = dbg!(dbg!((p_invalid / p_valid).ln()) + dbg!(oo)) > 1_f64;
        dbg!((test_mail, is_spam));
        let valid_score = tokens.iter().fold(dbg!(p_valid.ln()), |a, k| {
            let valid = p_f_valid.get(*k).unwrap_or(&0.0000001_f64) + 0_f64;
            // dbg!((k, valid, (valid).ln(), a + valid.ln()));
            a + valid.ln()
        });
        let invalid_score = tokens.iter().fold(dbg!(p_invalid.ln()), |a, k| {
            let invalid = p_f_invalid.get(*k).unwrap_or(&0.0000001_f64) + 0_f64;
            // dbg!((k, invalid, (invalid).ln(), a + invalid.ln()));
            a + invalid.ln()
        });
        dbg!((
            test_mail,
            valid_score,
            invalid_score,
            1_f64.exp().powf(valid_score),
            1_f64.exp().powf(invalid_score)
        ));
    }
}
fn normalize(table: &HashMap<String, u64>) -> HashMap<String, f64> {
    let mut norm_table: HashMap<String, f64> = HashMap::new();
    let max = table.iter().fold(0_f64, |a, (_, v)| a + *v as f64);
    for (k, v) in table {
        if let Some(o) = norm_table.get_mut(k) {
            *o += (*v as f64) / max;
        } else {
            norm_table.insert(k.into(), (*v as f64) / max);
        }
    }
    norm_table
}
fn count(mails: &[&str]) -> HashMap<String, u64> {
    let mut token_table: HashMap<String, u64> = HashMap::new();
    for mail in mails {
        for token in mail.split(" ") {
            if let Some(pntr) = token_table.get_mut(token) {
                *pntr += 1;
            } else {
                token_table.insert(token.into(), 1);
            }
        }
    }
    token_table
}
