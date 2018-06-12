extern crate futures;
extern crate rusoto_core;
extern crate rusoto_s3;

use futures::Future;
use rusoto_core::Region;
use rusoto_core::credential::AwsCredentials;
use rusoto_core::reactor::CredentialsProvider;
use rusoto_s3::GetObjectRequest;
use rusoto_s3::util::PreSignedRequest;

fn main() {
    let region = Region::ApNortheast1;
    let credentials = AwsCredentials::new(
        "foo",
        "bar",
        None,
        None
    );
    let bucket = "mybucket".to_string();
    let filename = "foo.png".to_string();
    let req = GetObjectRequest {
        bucket: bucket.to_string(),
        key: filename.to_string(),
        ..Default::default()
    };
    let presigned_url = req.get_presigned_url(&region, &credentials);
    println!("{:?}", presigned_url);
}
