use std::time::Duration;
use std::collections::HashMap;
use mdo::mdo;
use mdo_future::future::*;
use futures::prelude::*;
use failure::{Error, format_err};
use rusoto_credential::AwsCredentials;
use rusoto_core::region::Region;
use rusoto_cognito_identity::{CognitoIdentity, CognitoIdentityClient, GetOpenIdTokenForDeveloperIdentityInput};
use rusoto_sts::{Sts, StsClient, AssumeRoleWithWebIdentityRequest};
use rusoto_s3::{GetObjectRequest};
use rusoto_s3::util::{PreSignedRequest, PreSignedRequestOption};


#[derive(serde_derive::Deserialize, Debug, Clone)]
struct Config {
    identity_pool_id: String,
    identity_pool_provider: String,
    image_bucket_name: String,
    image_bucket_get_role_arn: String,
    aws_access_key_id: String,
    aws_secret_access_key: String,
    aws_region: String,
}

fn main() {
    dotenv::dotenv().ok();
    let config = envy::from_env::<Config>().unwrap();
    println!("{:?}", config);
    let identity_name = "device_0".to_string();
    let filename = "device_setting.txt".to_string();
    let filepath = format!("{}/{}", identity_name.clone(), filename.clone());
    let cognito_client = CognitoIdentityClient::new(Region::default());
    let sts_client = StsClient::new(Region::default());
    let fut: Box<dyn Future<Item=(), Error=Error> + Send + 'static> = Box::new(mdo!{
        let logins = {
            let mut logins = HashMap::new();
            logins.insert(config.identity_pool_provider.clone(), identity_name.clone());
            logins
        };
        tokens =<< cognito_client
            .get_open_id_token_for_developer_identity(
                GetOpenIdTokenForDeveloperIdentityInput{
                    identity_id: None,
                    token_duration: None,
                    identity_pool_id: config.identity_pool_id.clone(),
                    logins,
                    ..Default::default()
                }
            )
            .map_err(Into::into);
        let _ = println!("token: {:?}", tokens);
        web_identity_token =<< tokens.token.ok_or(format_err!("missing token")).into_future();
        let policy = format!(r###"{{
    "Version": "2012-10-17",
    "Statement": [
        {{
            "Effect": "Allow",
            "Action": ["s3:ListBucket", "s3:GetObject"],
            "Resource": "arn:aws:s3:::{bucket_name}/{filepath}"
        }}
    ]
}}"###, bucket_name=config.image_bucket_name.clone(), filepath=filepath.clone());
        let _ = println!("policy: {}", policy);
        creds =<< sts_client
            .assume_role_with_web_identity(
                AssumeRoleWithWebIdentityRequest{
                    duration_seconds: None,
                    policy: Some(policy),
                    provider_id: None,
                    role_arn: config.image_bucket_get_role_arn.clone(),
                    role_session_name: "dev".to_string(),
                    web_identity_token,
                }
            )
            .map_err(Into::into);
        let () = println!("creds: {:?}", creds);
        creds =<< creds.credentials.ok_or(failure::err_msg("missing credentials")).into_future();
        let _ = println!("{}", creds.expiration);
        let creds = AwsCredentials::new(
            creds.access_key_id,
            creds.secret_access_key,
            Some(creds.session_token),
            None,
        );
        
        let req = GetObjectRequest {
            bucket: config.image_bucket_name.clone(),
            key: filepath.to_string(),
            ..Default::default()
        };
        let presigned_url = req.get_presigned_url(&Region::default(), &creds, &PreSignedRequestOption{
            expires_in: Duration::from_secs(120)
        });
        let _ = println!("presigned_url: {}", presigned_url);
        ret ret(())
    });
    tokio::run(fut.map_err(|e| eprintln!("{:?}", e)));

}
