use tokio::runtime::current_thread::block_on_all;
use std::process::Command;
use chrono::prelude::*;
use chrono::naive::NaiveDateTime;
#[derive(serde_derive::Deserialize, Debug, Clone)]
struct Config{
    consumer_key: String,
    consumer_secret: String,
    access_token: String,
    access_secret: String,
    screen_name: String,
}

fn main() {
    dotenv::dotenv().ok();
    ::env_logger::try_init().ok();
    let config = envy::from_env::<Config>().unwrap();
    println!("config: {:?}", config);
    let token = egg_mode::Token::Access {
        consumer: egg_mode::KeyPair::new(config.consumer_key, config.consumer_secret),
        access: egg_mode::KeyPair::new(config.access_token, config.access_secret),
    };
    loop{
        let (_, res) = block_on_all(egg_mode::tweet::liked_by(&config.screen_name, &token).with_page_size(1024).start()).unwrap();
        for tw in &res {
            if let &Some(ref user) = &tw.user {
                let created_at = {
                    let c = &tw.created_at;
                    format!("{}-{}-{}_{}-{}-{}", c.year() as u32, c.month() as u32, c.day() as u32, c.hour() as u32, c.minute() as u32, c.second() as u32)
                };
                println!("{}@{}:{}:{}", user.name, user.screen_name, tw.id, created_at);
                println!("\thttps://mobile.twitter.com/{}/status/{}", user.screen_name, tw.id);
                println!("\ttext: {}", tw.text);
                if let &Some(ref entiry) = &tw.extended_entities {
                    let media = &entiry.media;
                    let mut i = 0;
                    for entity in media {
                        let ext  = entity.media_url_https.rsplitn(2, '.').take(1).next().unwrap();
                        println!("\t{}:{:?}:{}", i, entity.media_url_https, ext);
                        if !(ext == "jpg" || ext == "png") {
                            continue;
                        }
                        let foldername = format!("/home/legokichi/Dropbox/tw/{}", user.screen_name);
                        let _output = Command::new("mkdir")
                            .arg("-p")
                            .arg(foldername)
                            .output()
                            .unwrap();
                        let filename = format!("/home/legokichi/Dropbox/tw/{}/{}_{}-{}-{}.{}", user.screen_name, created_at, user.screen_name, tw.id, i, ext);
                        println!("\t{}:{:?}", i, filename);
                        let output = Command::new("curl")
                            .arg(&entity.media_url_https)
                            .arg("-o")
                            .arg(filename)
                            .output()
                            .unwrap();
                        if !output.status.success() {
                            panic!("{:?}", output.status.code());
                        }
                        println!("\t{}...exit_code:{:?}", i, output.status.code());
                        i += 1;
                    }
                    let o = block_on_all(egg_mode::tweet::unlike(tw.id, &token));
                    println!("\tunlike: {}", o.is_ok());
                }
            }
        }
        println!("rate_limit: {}/{}, reset: {}", res.rate_limit_remaining, res.rate_limit, NaiveDateTime::from_timestamp(res.rate_limit_reset.into(), 0));
        if res.rate_limit_remaining < res.rate_limit/10  {
            break;
        }
        ::std::thread::sleep(::std::time::Duration::from_secs(60*30));
    }
}
