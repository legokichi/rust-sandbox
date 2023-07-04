use rusoto_s3::S3;

#[tokio::main]
async fn main() {
    let bucket = "actcast-dev-firmware-release-version";
    let key = "1.0.0-72187+220630.f6f773f/raspberrypi.jso";
    let s3 = rusoto_s3::S3Client::new(rusoto_core::Region::ApNortheast1);
    let ret = s3.get_object(rusoto_s3::GetObjectRequest {
                    bucket: String::from(bucket),
                    key: String::from(key),
                    ..Default::default()
                }
            )
            .await;
    println!("{ret:?}");
}
