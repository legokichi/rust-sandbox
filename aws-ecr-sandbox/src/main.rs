#![allow(unused_imports)]
extern crate failure;
extern crate futures;
extern crate tokio;
#[macro_use]
extern crate mdo;
extern crate mdo_future;
extern crate rusoto_core;
extern crate rusoto_credential;
extern crate rusoto_cognito_identity;
extern crate rusoto_sts;
extern crate rusoto_ecr;

use failure::Fail;
use futures::future;
use futures::prelude::*;
use mdo_future::future::*;
use std::str::FromStr;
use std::collections::HashMap;
use rusoto_core::credential::AwsCredentials;
use rusoto_core::region::Region;
use rusoto_core::request::DispatchSignedRequest;
use rusoto_core::request::HttpClient;
use rusoto_credential::ProvideAwsCredentials;
use rusoto_credential::EnvironmentProvider;
use rusoto_cognito_identity::{CognitoIdentity, CognitoIdentityClient, GetOpenIdTokenForDeveloperIdentityInput};
use rusoto_sts::{Sts, StsClient, AssumeRoleWithWebIdentityRequest};

fn main() {
    let region = Region::from_str(
        &::std::env::var("AWS_REGION")
            .expect(&format!("AWS_REGION is undefined in env"))
    )
        .expect(&format!("invalid AWS_REGION"));
    
    let fut = mdo!{
        let logins = {
            let mut logins = HashMap::new();
            logins.insert("login.yosuke_ino_devices".to_string(), "device_0".to_string());
            logins
        };
        let identity_pool_id = "ap-northeast-1:118d78d4-a0f7-4fe8-b049-0f66cbe2b3d7".to_string();
        let cognito_cli = CognitoIdentityClient::new(region.clone());
        tokens =<< cognito_cli
            .get_open_id_token_for_developer_identity(
                GetOpenIdTokenForDeveloperIdentityInput{
                    identity_id: None,
                    identity_pool_id,
                    logins,
                    token_duration: None,
                }
            )
            .map_err(Into::into);
        let () = println!("{:?}", tokens);
        // do not start \n
        let policy = r###"{
	"Version": "2012-10-17",
	"Statement": [{
		"Effect": "Allow",
		"Action": [
			"ecr:GetAuthorizationToken", "ecr:BatchCheckLayerAvailability", "ecr:GetRepositoryPolicy", "ecr:DescribeRepositories", "ecr:ListImages", "ecr:DescribeImages", "ecr:DescribeRepositories", "ecr:BatchGetImage"
		],
		"Resource": "*"
	}, {
		"Effect": "Allow",
		"Action": "ecr:GetDownloadUrlForLayer",
		"Resource": "arn:aws:ecr:ap-northeast-1:746316586548:repository/test/yosuke-ino"
	}]
}"###.to_string();
        let sts_cli = StsClient::new(region.clone());
        creds =<< sts_cli
            .assume_role_with_web_identity(
                AssumeRoleWithWebIdentityRequest{
                    duration_seconds: None,
                    policy: Some(policy),
                    provider_id: None,
                    role_arn: "arn:aws:iam::746316586548:role/yosuke_ino_devices_role".to_string(),
                    role_session_name: "yosuke_ino_test".to_string(),
                    web_identity_token: tokens.token.unwrap(),
                }
            )
            .map_err(Into::into);
        let () = println!("{:?}", creds);
        let creds = creds.credentials.unwrap();
        let access_key_id = creds.access_key_id;
        let secret_access_key = creds.secret_access_key;
        let session_token  = creds.session_token;
        let () = {
            println!("export AWS_ACCESS_KEY_ID={}", access_key_id);
            println!("export AWS_SECRET_ACCESS_KEY={}", secret_access_key);
            println!("export AWS_SESSION_TOKEN={}", session_token);
            println!("export AWS_REGION={}", region.name());
        };
        ret ret(())
    };
    tokio::run(fut.map_err(|err: failure::Error| println!("{:?}", err)));
}
