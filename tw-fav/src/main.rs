#![feature(async_await, async_closure)]

use chrono::naive::NaiveDateTime;
use chrono::prelude::*;
use futures::compat::{Future01CompatExt as _, Stream01CompatExt as _};
use futures_util::try_stream::TryStreamExt as _;
use log::*;
use serde::Deserialize;
use std::error::Error;
use std::time::Duration;

#[derive(Deserialize, Debug, Clone)]
struct Config {
    consumer_key: String,
    consumer_secret: String,
    access_token: String,
    access_secret: String,
    screen_name: String,
}

#[runtime::main(runtime_tokio::Tokio)]
async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "info");
    env_logger::try_init().ok();
    let config = envy::from_env::<Config>().unwrap();

    let token = egg_mode::Token::Access {
        consumer: egg_mode::KeyPair::new(config.consumer_key, config.consumer_secret),
        access: egg_mode::KeyPair::new(config.access_token, config.access_secret),
    };
    loop {
        let (mut timeline, mut res) =
        egg_mode::tweet::liked_by(&config.screen_name, &token)
        // egg_mode::tweet::user_timeline("ayaoricarbon", false, false, &token)
            .with_page_size(50)
            .start()
            .compat()
            .await?;
        loop {
            let len = res.len();
            let results: Vec<Result<String, _>> =
                futures::future::join_all(res.iter().enumerate().map({
                    let token = &token;
                    async move |(h, tw)| -> Result<String, Box<dyn Error + Send + Sync + 'static>> {
                        let mut logs: Vec<String> = Vec::new();
                        logs.push(format!("{}/{}", h, len));
                        // logs.push(format!("{:?}", tw));
                        if let &Some(ref user) = &tw.user {
                            let created_at = {
                                let c = &tw.created_at;
                                format!(
                                    "{}-{}-{}_{}-{}-{}",
                                    c.year() as u32,
                                    c.month() as u32,
                                    c.day() as u32,
                                    c.hour() as u32,
                                    c.minute() as u32,
                                    c.second() as u32
                                )
                            };
                            logs.push(format!(
                                "{}@{}:{}:{}",
                                user.name, user.screen_name, tw.id, created_at
                            ));
                            logs.push(format!(
                                "\thttps://mobile.twitter.com/{}/status/{}",
                                user.screen_name, tw.id
                            ));
                            logs.push(format!("\ttext: {}", tw.text));
                            if let &Some(ref entiry) = &tw.extended_entities {
                                let media = &entiry.media;
                                let mut i = 0;
                                for entity in media {
                                    let ext = entity
                                        .media_url_https
                                        .rsplitn(2, '.')
                                        .take(1)
                                        .next()
                                        .unwrap();
                                    logs.push(format!(
                                        "\t{}:{:?}:{}",
                                        i, entity.media_url_https, ext
                                    ));
                                    if !(ext == "jpg" || ext == "png") {
                                        continue;
                                    }
                                    let foldername =
                                        format!("/home/legokichi/Dropbox/tw/{}", user.screen_name);
                                    logs.push(format!("\t{}: mkdir -p {} ", i, foldername));
                                    tokio_fs::create_dir_all(foldername).compat().await?;
                                    let filename = format!(
                                        "/home/legokichi/Dropbox/tw/{}/{}_{}-{}-{}.{}",
                                        user.screen_name,
                                        created_at,
                                        user.screen_name,
                                        tw.id,
                                        i,
                                        ext
                                    );
                                    logs.push(format!(
                                        "\t{}: curl {} -o {}",
                                        i, entity.media_url_https, filename
                                    ));
                                    let client = reqwest::r#async::ClientBuilder::new().build()?;
                                    let res =
                                        client.get(&entity.media_url_https).send().compat().await?;
                                    logs.push(format!("\tstatus code: {}", res.status()));
                                    if res.status().is_success() {
                                        let body = res.into_body().compat().try_concat().await?;
                                        tokio::fs::write(filename, body).compat().await?;
                                    } else {
                                        Err("download failed. skip it")?;
                                    }
                                    i += 1;
                                }
                                let o = egg_mode::tweet::unlike(tw.id, token).compat().await;
                                logs.push(format!("\tunlike: {}", o.is_ok()));
                            }
                        }
                        Ok(logs.join("\n"))
                    }
                }))
                .await;
            for result in results {
                match result {
                    Ok(result) => {
                        info!("{}", result);
                    }
                    Err(err) => {
                        error!("{}", err);
                    }
                }
            }
            info!(
                "rate_limit: {}/{}, reset: {}",
                res.rate_limit_remaining,
                res.rate_limit,
                NaiveDateTime::from_timestamp(res.rate_limit_reset.into(), 0)
            );
            let min_id = timeline.min_id;
            info!(
                "min_id: {:?}, max_id: {:?}, count: {}",
                timeline.min_id, timeline.max_id, timeline.count
            );
            let o = timeline.older(None).compat().await?;
            timeline = o.0;
            res = o.1;
            if timeline.count == 0 || min_id.is_none() {
                break;
            }
            if res.rate_limit_remaining < res.rate_limit / 10 {
                break;
            }
        }
        info!("wait"); 
        runtime::time::Delay::new(Duration::from_secs(60 * 30)).await;
    }
    Ok(())
}
