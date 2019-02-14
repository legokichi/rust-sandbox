#[macro_use]
extern crate serde_derive;
extern crate quickcheck_sandbox as qdb;
#[macro_use]
extern crate proptest;
// #[macro_use]
// extern crate quickcheck;
#[macro_use]
extern crate log;
#[macro_use]
extern crate failure;

// use quickcheck::{TestResult, quickcheck};
use chrono::{NaiveDateTime, Utc};
use qdb::Db;
use qdb::tx;
use qdb::models::*;
use failure::Error;
use proptest::prelude::*;
use try_from::*;

#[derive(Clone, Debug, Deserialize)]
struct Config {
    database_url: String,
}


// quickcheck! {
//     fn quickcheck_main(all_data: usize, offset: usize, limit: usize) -> bool {
//         dotenv::dotenv().unwrap();
//         env_logger::try_init().ok();
//         let config = envy::from_env::<Config>().unwrap();

//         println!("all_data: {}, offset: {}, limit: {}", all_data, offset, limit);
//         info!("config: {:?}", config);
//         let db = Db::new(&config.database_url).unwrap();

//         let datas = (0..all_data).into_iter()
//             .map(|i| (i, Utc::now(), "".to_string(), "".to_string()))
//             .collect::<Vec<_>>();
//         let posts = datas.iter()
//             .map(|(_id, now, author, body)| NewPost{ author, body, timestamp: NaiveDateTime::from_timestamp(now.timestamp(), 0)})
//             .collect::<Vec<_>>();

//         macro_rules! assert_eq {
//             ($left:expr, $right:expr) => (
//                 if $left != $right {
//                     error!("failed @ {} == {} as {:?} == {:?}", stringify!($left), stringify!($right), $left, $right);
//                     return Ok(false);
//                 }
//             )
//         }
//         let flag = db.test_transaction(|conn|{
//             use std::cmp::{max, min};
//             assert_eq!(posts.len(), datas.len());
//             assert_eq!(posts.len() as u64, all_data as u64);

//             let () = tx::insert_posts(conn, &posts)?;

//             let (list, count) = tx::list_posts_with_limit(conn, offset as i64, limit as i64)?;
//             assert_eq!(count, all_data as i64);
//             assert_eq!(list.len() as i64, min(max(count - offset as i64, 0), limit as i64));

//             let (list2, count2) = tx::list_posts(conn)?;
//             assert_eq!(list2.len() as i64, count2);
//             assert_eq!(count, count2);
//             Ok(true)
//         });
//         flag
//     }
// }
proptest! {
    #[test]
    fn proptest_main(all_data: u16, offset: u64, limit: u64) {

        dotenv::dotenv().unwrap();
        env_logger::try_init().ok();
        let config = envy::from_env::<Config>().unwrap();

        info!("all_data: {}, offset: {}, limit: {}", all_data, offset, limit);
        let db = Db::new(&config.database_url).unwrap();

        let datas = (0..all_data).into_iter()
            .map(|i| (i, Utc::now(), "".to_string(), "".to_string()))
            .collect::<Vec<_>>();
        let posts = datas.iter()
            .map(|(_id, now, author, body)| NewPost{ author, body, timestamp: NaiveDateTime::from_timestamp(now.timestamp(), 0)})
            .collect::<Vec<_>>();

        db.test_transaction(|conn|{
            use std::cmp::{max, min};
            assert_eq!(posts.len(), datas.len());
            assert_eq!(posts.len(), all_data as usize);

            let () = tx::insert_posts(conn, &posts)?;

            let (list, count) = tx::list_posts_with_limit(conn, offset, limit)?;
            assert_eq!(count, all_data as i64);
            assert_eq!(list.len() as i64, min(max(count - offset as i64, 0), limit as i64));

            let (list2, count2) = tx::list_posts(conn)?;
            assert_eq!(list2.len() as i64, count2);
            assert_eq!(count, count2);
            Ok(())
        })
    }
}

#[derive(Clone, Debug)]
struct Order {
  id: String,
  item: String,
  quantity: u32,
}

fn vec_and_index() -> impl Strategy<Value = (Vec<u8>, usize)> {
    prop::collection::vec(any::<u8>(), 1..10)
        .prop_flat_map(|vec| {
            let len = vec.len();
            (Just(vec), 0..len)
        })
}

proptest!{
    #[test]
    fn fa_test(o in vec_and_index()){
        println!("{:?}", o);
        
    }
}

// #[test]
// fn o(){
//     fn prop(length: usize, index: usize) -> TestResult {
//         let v: Vec<_> = (0..length).collect();
//         if index < length {
//             TestResult::discard()
//         } else {
//             TestResult::must_fail(move || {
//                 v[index]
//             })
//         }
//     }
//     quickcheck(prop as fn(usize, usize) -> TestResult);
// }


// #[cfg(test)]
// mod pt_tests {
//     use proptest::prelude::*;
//     proptest! {
//         #[test]
//         fn doesnt_crash(s in "\\PC*") {
//             println!("{}", s);
//         }
//     }
// }


// mod tests {
//     fn reverse<T: Clone>(xs: &[T]) -> Vec<T> {
//         let mut rev = vec!();
//         for x in xs.iter() {
//             rev.insert(0, x.clone())
//         }
//         rev
//     }
    
//     quickcheck! {
//         fn prop(xs: Vec<u32>) -> bool {
//             xs == reverse(&reverse(&xs))
//         }
//     }
// }
