extern crate futures;
extern crate rusoto_core;
extern crate rusoto_s3;

use futures::prelude::*;
use rusoto_core::Region;
use rusoto_core::ProvideAwsCredentials;
use rusoto_core::DefaultCredentialsProvider;
use rusoto_s3::PutObjectRequest;
use rusoto_s3::util::PreSignedRequest;
use rusoto_s3::util::PreSignedRequestOption;
use std::time::Duration;

fn main() {
    let region = Region::ApNortheast1;
    let credentials = DefaultCredentialsProvider::new().unwrap().credentials().wait().unwrap();

    let bucket = "image.devices-xxxxxx.xxxxxx.xxxxxx.io".to_string();
    let filename = "device_setting.txt".to_string();
    let req = PutObjectRequest {
        bucket: bucket.to_string(),
        key: filename.to_string(),
        ..Default::default()
    };
    let presigned_url = req.get_presigned_url(&region, &credentials, &PreSignedRequestOption{
        expires_in: Duration::from_secs(120)
    });
    println!("{:?}", presigned_url);

    let req = PutObjectRequest {
        bucket: bucket.to_string(),
        key: filename.to_string(),
        content_type: Some("image/jpeg".to_string()),
        ..Default::default()
    };
    let presigned_url = req.get_presigned_url(&region, &credentials, &PreSignedRequestOption{
        expires_in: Duration::from_secs(120)
    });
    println!("{:?}", presigned_url);
}
