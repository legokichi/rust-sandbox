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
use rusoto_ecr::{Ecr, EcrClient, GetAuthorizationTokenRequest};

fn main() {
    let identity_pool_id = ::std::env::var("IDENTITY_POOL_ID").unwrap();
    let ecr_repo_arn = ::std::env::var("ECR_REPO_ARN").unwrap();
    let registory_id = ::std::env::var("REGISTORY_ID").unwrap();
    let role_arn = ::std::env::var("ROLE_ARN").unwrap();
    let custom_provider = ::std::env::var("IDENTITY_POOL_PROVIDER").unwrap();
    let region = Region::from_str(
        &::std::env::var("AWS_REGION")
            .expect(&format!("AWS_REGION is undefined in env"))
    )
        .expect(&format!("invalid AWS_REGION"));
    
    let fut = mdo!{
        let logins = {
            let mut logins = HashMap::new();
            logins.insert(custom_provider, "device_0".to_string());
            logins
        };
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
        let policy = format!(r###"{{
	"Version": "2012-10-17",
	"Statement": [{{
		"Effect": "Allow",
		"Action": [
			"ecr:GetAuthorizationToken", "ecr:BatchCheckLayerAvailability", "ecr:GetRepositoryPolicy",
            "ecr:DescribeRepositories", "ecr:ListImages", "ecr:DescribeImages", "ecr:DescribeRepositories",
            "ecr:BatchGetImage"
		],
		"Resource": "*"
	}}, {{
		"Effect": "Allow",
		"Action": "ecr:GetDownloadUrlForLayer",
		"Resource": "{}"
	}}]
}}"###, ecr_repo_arn);
        let sts_cli = StsClient::new(region.clone());
        creds =<< sts_cli
            .assume_role_with_web_identity(
                AssumeRoleWithWebIdentityRequest{
                    duration_seconds: None,
                    policy: Some(policy),
                    provider_id: None,
                    role_arn,
                    role_session_name: "dev".to_string(),
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
        let ecr_cli = EcrClient::new(region.clone());
        data =<< ecr_cli
            .get_authorization_token(GetAuthorizationTokenRequest{
                registry_ids: Some(vec![registory_id])
            })
            .map_err(Into::into);
        let () = println!("{:?}", data);
        ret ret(())
    };
    tokio::run(fut.map_err(|err: failure::Error| println!("{:?}", err)));
}
