#[tokio::main]
async fn main() {
    use rusoto_iot::Iot;
    let region = rusoto_core::Region::Custom {
        name: "local".to_string(),
        endpoint: "http://localhost:8080/".to_string(),
    };
    //let region = rusoto_core::Region::ApNortheast1;
    let client = rusoto_iot::IotClient::new(region);
    let ret = client
        .list_thing_principals(rusoto_iot::ListThingPrincipalsRequest {
            thing_name: "5bc02ff0-decf-4eda-bc25-9589dae1b419".to_string(),
            next_token: None,
            max_results: None,
        })
        .await;
    println!("{ret:?}");
}
