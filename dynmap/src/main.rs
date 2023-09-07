#[tokio::main]
async fn main() {
    let timestamp = chrono::Utc::now().timestamp_millis();
    loop {
        let url = format!("https://minecraft-map.sudosan.net/up/world/world/{timestamp}");
        let res = reqwest::get(url).await;
        let res = match res {
            Ok(res) => res,
            Err(err) => {
                println!("error: {:?}", err);
                tokio::time::sleep(std::time::Duration::from_secs(30)).await;
                continue;
            }
        };
        if res.status() != reqwest::StatusCode::OK {
            println!("error: {}", res.status());
            for header in res.headers() {
                println!("{}: {}", header.0, header.1.to_str().unwrap());
            }
            println!("{}", res.text().await.unwrap());
            tokio::time::sleep(std::time::Duration::from_secs(30)).await;
            continue;
        }
        let json: serde_json::Value = res.json().await.unwrap();
        //println!("{}", serde_json::to_string_pretty(&json).unwrap());
        let timestamp = json.pointer("/timestamp").unwrap().as_i64().unwrap();
        let is_thundering = json.pointer("/isThundering").unwrap().as_bool().unwrap();
        let has_storm = json.pointer("/hasStorm").unwrap().as_bool().unwrap();
        let date_str = {
            use chrono::TimeZone;
            let dt: chrono::DateTime<chrono::Utc> = chrono::Utc
                .timestamp_millis_opt(timestamp)
                .single()
                .unwrap();
            let dt: chrono::DateTime<chrono::FixedOffset> =
                dt.with_timezone(&chrono::FixedOffset::east_opt(9 * 3600).unwrap());
            dt.to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
        };
        if is_thundering && has_storm {
            println!("{date_str}\tthunderstorm");
        }
        let players = json
            .pointer("/players")
            .unwrap()
            .as_array()
            .unwrap()
            .iter()
            .collect::<Vec<_>>();
        for player in players {
            let world = player.pointer("/world").unwrap().as_str().unwrap();
            let x = player.pointer("/x").unwrap().as_f64().unwrap();
            let y = player.pointer("/y").unwrap().as_f64().unwrap();
            let z = player.pointer("/z").unwrap().as_f64().unwrap();
            let name = player.pointer("/name").unwrap().as_str().unwrap();
            let account = player.pointer("/account").unwrap().as_str().unwrap();
            println!("{date_str}\tplayer\t{account}\t{name}\t{world}\t{x}\t{y}\t{z}");
        }
        let updates = json
            .pointer("/updates")
            .unwrap()
            .as_array()
            .unwrap()
            .iter()
            .collect::<Vec<_>>();
        for update in updates {
            let r#type = update.pointer("/type").unwrap().as_str().unwrap();
            if r#type == "chat" {
                let timestamp = update.pointer("/timestamp").unwrap().as_i64().unwrap();
                let account = update.pointer("/account").unwrap().as_str().unwrap();
                let name = update.pointer("/playerName").unwrap().as_str().unwrap();
                let message = update.pointer("/message").unwrap().as_str().unwrap();
                let date_str = {
                    use chrono::TimeZone;
                    let dt: chrono::DateTime<chrono::Utc> = chrono::Utc
                        .timestamp_millis_opt(timestamp)
                        .single()
                        .unwrap();
                    let dt: chrono::DateTime<chrono::FixedOffset> =
                        dt.with_timezone(&chrono::FixedOffset::east_opt(9 * 3600).unwrap());
                    dt.to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
                };
                println!("{date_str}\tchat\t{account}\t{name}\t{message}");
            } else if r#type == "playerjoin" {
                let timestamp = update.pointer("/timestamp").unwrap().as_i64().unwrap();
                let account = update.pointer("/account").unwrap().as_str().unwrap();
                let name = update.pointer("/playerName").unwrap().as_str().unwrap();
                let date_str = {
                    use chrono::TimeZone;
                    let dt: chrono::DateTime<chrono::Utc> = chrono::Utc
                        .timestamp_millis_opt(timestamp)
                        .single()
                        .unwrap();
                    let dt: chrono::DateTime<chrono::FixedOffset> =
                        dt.with_timezone(&chrono::FixedOffset::east_opt(9 * 3600).unwrap());
                    dt.to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
                };
                println!("{date_str}\tjoin\t{account}\t{name}");
            } else if r#type == "playerquit" {
                let timestamp = update.pointer("/timestamp").unwrap().as_i64().unwrap();
                let account = update.pointer("/account").unwrap().as_str().unwrap();
                let name = update.pointer("/playerName").unwrap().as_str().unwrap();
                let date_str = {
                    use chrono::TimeZone;
                    let dt: chrono::DateTime<chrono::Utc> = chrono::Utc
                        .timestamp_millis_opt(timestamp)
                        .single()
                        .unwrap();
                    let dt: chrono::DateTime<chrono::FixedOffset> =
                        dt.with_timezone(&chrono::FixedOffset::east_opt(9 * 3600).unwrap());
                    dt.to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
                };
                println!("{date_str}\tquit\t{account}\t{name}");
            } else if r#type == "tile" {
                // nop
            } else if r#type == "daynight" {
                let timestamp = update.pointer("/timestamp").unwrap().as_i64().unwrap();
                let is_day = update.pointer("/isday").unwrap().as_bool().unwrap();
                let date_str = {
                    use chrono::TimeZone;
                    let dt: chrono::DateTime<chrono::Utc> = chrono::Utc
                        .timestamp_millis_opt(timestamp)
                        .single()
                        .unwrap();
                    let dt: chrono::DateTime<chrono::FixedOffset> =
                        dt.with_timezone(&chrono::FixedOffset::east_opt(9 * 3600).unwrap());
                    dt.to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
                };
                if is_day {
                    println!("{date_str}\tday");
                }else {
                    println!("{date_str}\tnight");
                }
            } else {
                println!("{}", serde_json::to_string_pretty(&update).unwrap());
            }
        }
        tokio::time::sleep(std::time::Duration::from_secs(30)).await;
    }
}
