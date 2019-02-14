use chrono::{NaiveDateTime, Utc};
use diesel::debug_query;
use diesel::prelude::*;
use diesel::sqlite::Sqlite;
use crate::models::*;
use crate::Error;
use crate::schema::posts::dsl;
use try_from::TryInto;



pub fn create_post<'a>(
    conn: &SqliteConnection,
    author: &'a str,
    body: &'a str,
) ->  Result<(), Error> {
    let now = Utc::now();
    let new_post = NewPost {
        timestamp: NaiveDateTime::from_timestamp(now.timestamp(), 0),
        author,
        body,
    };
    insert_post(conn, &new_post)?;
    Ok(())
}

pub fn list_posts(
    conn: &SqliteConnection,
) -> Result<(Vec<Post>, i64), Error> {
    
    conn.transaction(|| {
        let query = dsl::posts
            .order(dsl::timestamp.desc());
        debug!("{}", debug_query::<Sqlite, _>(&query));
        let list = query.get_results::<Post>(conn).map_err(Error::Diesel)?;
        let query = dsl::posts.count();
        debug!("{}", debug_query::<Sqlite, _>(&query));
        let count = query.get_result(conn).map_err(Error::Diesel)?;
        Ok((list, count))
    })
}

pub fn list_posts_with_limit(
    conn: &SqliteConnection,
    offset: u64,
    limit: u64,
) -> Result<(Vec<Post>, i64), Error> {
    let offset = TryInto::<i64>::try_into(offset).map_err(Error::TryFromIntError)?;
    let limit = TryInto::<i64>::try_into(offset).map_err(Error::TryFromVoid)?;
    
    conn.transaction(|| {
        let query = dsl::posts
            .order(dsl::timestamp.desc())
            .limit(limit)
            .offset(offset);
        debug!("{}", debug_query::<Sqlite, _>(&query));
        let list = query.get_results::<Post>(conn).map_err(Error::Diesel)?;
        let query = dsl::posts.count();
        debug!("{}", debug_query::<Sqlite, _>(&query));
        let count = query.get_result(conn).map_err(Error::Diesel)?;
        Ok((list, count))
    }
    )
}

pub fn insert_post<'a>(conn: &SqliteConnection, post: &'a NewPost<'a>) -> Result<(), Error> {
    
    let query = diesel::insert_into(dsl::posts).values(post);
    debug!("{}", debug_query::<Sqlite, _>(&query));
    let insertions = query.execute(conn).map_err(Error::Diesel)?;
    assert_eq!(insertions, 1);
    Ok(())
}

pub fn insert_posts<'a>(conn: &SqliteConnection, posts: &'a [NewPost<'a>]) -> Result<(), Error> {
    
    let query = diesel::insert_into(dsl::posts).values(posts);
    let insertions = query.execute(conn).map_err(Error::Diesel)?;
    assert_eq!(insertions, posts.len());
    Ok(())
}

pub enum Vote {
    Like, Unlike
}

pub fn update_post(conn: &SqliteConnection, id: i32, vote: Vote) -> Result<(), Error> {
    
    conn.transaction(|| {
        let query = dsl::posts.find(id);
        debug!("{}", debug_query::<Sqlite, _>(&query));
        let opt_post = query.get_result::<Post>(conn)
            .optional().map_err(Error::Diesel)?;
        if let Some(post) = opt_post {
            let patch = match vote {
                Vote::Like => UpdatePost {
                    like: Some(post.like + 1),
                    unlike: None,
                },
                Vote::Unlike => UpdatePost {
                    like: None,
                    unlike: Some(post.unlike + 1),
                },
            };
            let query = diesel::update(dsl::posts.find(id)).set(&patch);
            debug!("{}", debug_query::<Sqlite, _>(&query));
            let updated_rows = query
                .execute(conn)
                .map_err(Error::Diesel)?;
            assert_eq!(updated_rows, 1);
            Ok(())
        }else{
            Err(Error::NotFound(id))
        }
    })
}

// #[cfg(test)]
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
