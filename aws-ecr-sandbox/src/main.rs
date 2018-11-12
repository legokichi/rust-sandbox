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
extern crate hyper_tls;
extern crate dotenv;
extern crate envy;
extern crate serde;
#[macro_use]
extern crate serde_derive;

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
use rusoto_credential::StaticProvider;
use rusoto_cognito_identity::{CognitoIdentity, CognitoIdentityClient, GetOpenIdTokenForDeveloperIdentityInput};
use rusoto_sts::{Sts, StsClient, AssumeRoleWithWebIdentityRequest};
use rusoto_ecr::{Ecr, EcrClient, GetAuthorizationTokenRequest};



#[derive(Deserialize, Debug, Clone)]
struct Config {
    identity_pool_id: String,
    identity_pool_provider: String,
    ecr_repo_arn: String,
    registory_id: String,
    role_arn: String,
    custom_provider: String,
    aws_access_key_id: String,
    aws_secret_access_key: String,
    aws_region: String,
}

fn main() {
    dotenv::dotenv().ok();
    let config = envy::from_env::<Config>().unwrap();
    let logins = {
        let mut logins = HashMap::new();
        logins.insert(config.custom_provider.clone(), "device_0".to_string());
        logins
    };
    let https_connector = ::hyper_tls::HttpsConnector::new(4).unwrap();
    let cred_provider =
        StaticProvider::new(config.aws_access_key_id.clone(), config.aws_secret_access_key.clone(), None, None);
    let cognito_cli = CognitoIdentityClient::new_with(
        HttpClient::from_connector(https_connector.clone()),
        cred_provider.clone(),
        Region::default(),
    );
    let sts_cli = StsClient::new_with(
        HttpClient::from_connector(https_connector.clone()),
        cred_provider.clone(),
        Region::default(),
    );
    let ecr_cli = EcrClient::new_with(
        HttpClient::from_connector(https_connector.clone()),
        cred_provider.clone(),
        Region::default(),
    );
    let fut: Box<Future<Item=(), Error=failure::Error> + Send + 'static> = Box::new(mdo!{
        tokens =<< cognito_cli
            .get_open_id_token_for_developer_identity(
                GetOpenIdTokenForDeveloperIdentityInput{
                    identity_id: None,
                    identity_pool_id: config.identity_pool_id.clone(),
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
}}"###, config.ecr_repo_arn.clone());
        web_identity_token =<< future::result(tokens.token.ok_or(failure::err_msg("missing token")));
        creds =<< sts_cli
            .assume_role_with_web_identity(
                AssumeRoleWithWebIdentityRequest{
                    duration_seconds: None,
                    policy: Some(policy),
                    provider_id: None,
                    role_arn: config.role_arn.clone(),
                    role_session_name: "dev".to_string(),
                    web_identity_token,
                }
            )
            .map_err(Into::into);
        let () = println!("{:?}", creds);
        creds =<< future::result(creds.credentials.ok_or(failure::err_msg("missing credentials")));
        let access_key_id = creds.access_key_id;
        let secret_access_key = creds.secret_access_key;
        let session_token  = creds.session_token;
        let () = {
            println!("export AWS_ACCESS_KEY_ID={}", access_key_id);
            println!("export AWS_SECRET_ACCESS_KEY={}", secret_access_key);
            println!("export AWS_SESSION_TOKEN={}", session_token);
            println!("export AWS_REGION={}", Region::default().name());
        };
        data =<< ecr_cli
            .get_authorization_token(GetAuthorizationTokenRequest{
                registry_ids: Some(vec![config.registory_id.clone()])
            })
            .map_err(Into::into);
        let () = println!("{:?}", data);
        ret ret(())
    });
    tokio::run(fut.map_err(|err: failure::Error| println!("{:?}", err)));
}
