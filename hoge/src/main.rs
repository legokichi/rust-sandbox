#![allow(unused_imports)]
#![allow(unreachable_code)]
#![allow(unused_variables)]
#![allow(dead_code)]

use frunk::Coproduct;
use frunk::{hlist, prelude::*, Coprod};
use futures::future::BoxFuture;
use futures::prelude::*;
use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient};

/*
[actlog]
 | validate
 +-------------------> [verr] ---report----x
[actlog] <-----+
 | IO query    | retry
 +-------------+-----> [qerr] ---report----x
([actlog], [table])
 | parse table
 +-------------------> [perr] ---report----x
([actlog], [device]) <--+
 | IO query             | retry
 +----------------------+--> [qerr] ---report----x
(req, [device], [table])
 | parse table
 +-----------------------> [perr] ---report----x
(req, [device],[act]) <----+
 | IO query                | retry
 +-------------------------+-> [qerr] ---report----x
(req, [device],[act],[table])
 | parse table
 +---------------------------> [perr] ---report----x
(req, [device],[act],[cast])
 | build reqa
 (....)
 | IO get bucket token <--+
 +------------------------+ [err]  ---report----x
(req, [device],[act],[cast], [req])
 | IO http req
 +---------------------------------...----------------x
(req, [device],[act],[cast], [req], [res])
 | build castlog
(req, [device],[act],[cast], [req], [res], [castlog])
 | IO put castlog    ^ retry
 +-------------------+-------------> perr ---report----x
(req, [device],[act],[cast], [req], [res], [castlog])
 |
 x
*/

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    lambda_runtime::run(lambda_runtime::handler_fn(|req, _| handler(req))).await.unwrap();
    Ok(())
}
#[derive(serde::Deserialize, serde::Serialize)]
struct Request{}
#[derive(serde::Deserialize, serde::Serialize)]
struct Response{}
async fn handler(req: Request) -> Result<Response, anyhow::Error> {
    let client = DynamoDbClient::new(Region::default());
    let req = pre_get_device(())?;
    let res = get_query(req).await;
    let c = after_get_device(res)?;
    let req = pre_list_act(())?;
    let res = list_act(req).await;
    let c = after_list_act(res)?;
    let req = before_list_cast()?;
    let res = futures::future::join(req.into_iter().map(|req| client.query(req) ).collect()).await;
    let c = RetType::inject(0_i32);
    let () = c
        .fold(hlist![
            |o: i32| async move {
                //                let o = io2(o).await;
                //                let o = pure3(o).unwrap();
                //                let o = io4(o).await.unwrap();
                //                let _ = pure4(o).unwrap();
            }
            .boxed(),
            |o: i64| async move {
                // エラー処理
                //                let o = io2_(o).await.unwrap();
                //                let _ = pure3_(o);
            }
            .boxed(),
        ])
        .await;
    Ok(Response{})
}

fn pre_get_device(req: ()) -> Result<(), anyhow::Error> {
    Ok(unimplemented!())
}
async fn get_device(req: ()) -> Result<Option<()>, aynhow::Error> {
}
fn after_get_device(
    o: Result<rusoto_dynamodb::QueryOutput, rusoto_core::RusotoError<rusoto_dynamodb::QueryError>>,
) -> Result<(), anyhow::Error> {
    Ok(())
}

fn pre_list_act(req: ()) -> Result<rusoto_dynamodb::QueryInput, anyhow::Error> {
    Ok(unimplemented!())
}

fn after_list_act(
    o: Result<rusoto_dynamodb::QueryOutput, rusoto_core::RusotoError<rusoto_dynamodb::QueryError>>,
) -> Result<(), anyhow::Error> {
    Ok(())
}

fn pre_list_cast(req: ()) -> Result<Vec<rusoto_dynamodb::QueryInput>, anyhow::Error> {
    Ok(unimplemented!())
}

fn after_list_cast(
    o: Result<rusoto_dynamodb::QueryOutput, rusoto_core::RusotoError<rusoto_dynamodb::QueryError>>,
) -> Result<(), anyhow::Error> {
    Ok(())
}
type RetType = Coprod!(i32, i64);
fn pure2(o: Result<(), anyhow::Error>) -> Result<RetType, anyhow::Error> {
    Ok(RetType::inject(0_i32))
}

fn io2(o: i32) -> impl Future<Output = Result<i32, anyhow::Error>> {
    futures::future::ready(Ok(0))
}
fn io2_(o: i64) -> impl Future<Output = Result<i64, anyhow::Error>> {
    futures::future::ready(Ok(0))
}
fn pure3(o: Result<i32, anyhow::Error>) -> Result<(), anyhow::Error> {
    Ok(())
}
fn pure3_(o: i64) -> Result<(), anyhow::Error> {
    Ok(())
}
fn io4(o: ()) -> impl Future<Output = Result<(), anyhow::Error>> {
    futures::future::ready(Ok(()))
}
fn pure4(o: ()) -> Result<(), anyhow::Error> {
    Ok(())
}
fn hnadler_error(o: ()) -> impl Future<Output = ()> {
    futures::future::ready(())
}


/*
課題


kinesis の req は Vec<Req>
Vec<Req> に対して最小限の リソース取得が必要
Vec<Req> -> IO Set<Device>
Set<Device> -> IO Map<ActId, Act>
Map<ActId, Act> -> IO Map<ActId, Set<Cast>>
(Vec<Req>, Set<Device>, Map<ActId, Act>, Map<ActId, Set<Cast>>) -> Vec<(&Req, &Device, &Act, &Cast)>
*/

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]

use serde::{Deserialize, Serialize};
use proptest::prelude::*;
#[macro_use]
extern crate proptest_derive;

#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error>{
    lambda_runtime::run(lambda_runtime::handler_fn(|req,_|{handler(req)})).await?;
    Ok(())
}
#[derive(Deserialize, Serialize, Arbitrary, Debug)]
struct Request{}
#[derive(Deserialize, Serialize, Arbitrary, Debug)]
struct Response{}
async fn handler(req: Request) -> Result<Response, anyhow::Error> {
    // let device = get_device(());
    // let act = get_device();
    Ok(unreachable!())
}
#[derive(Deserialize, Serialize, Arbitrary, Debug)]
struct Device{}
async fn get_device(req:()) -> Result<Option<Device>, anyhow::Error>{
    let a = pre_get_device(req)?;
    let b = io_get_device(a)?;
    let c = after_get_device(b)?;
    Ok(c)
}
fn pre_get_device(req: ()) -> Result<(), anyhow::Error>{
    // なんか引数チェック
    Ok(())
}
fn io_get_device(req: ()) -> Result<Option<serde_json::Value>, anyhow::Error>{
    // リトライとか
    Ok(None)
}
fn after_get_device(res: Option<serde_json::Value>) -> Result<Option<Device>, anyhow::Error>{
    // 後処理
    Ok(None)
}

#[derive(Deserialize, Serialize, Arbitrary, Debug)]
struct Act{}
async fn list_act(req:()) -> Result<Vec<Act>, anyhow::Error>{
    let a = pre_list_act(req)?;
    let b = io_list_act(a)?;
    let c = after_list_act(b)?;
    Ok(c)
}
fn pre_list_act(req: ()) -> Result<(), anyhow::Error>{
    // なんか引数チェック
    Ok(())
}
fn io_list_act(req: ()) -> Result<Vec<serde_json::Value>, anyhow::Error>{
    // リトライとか
    Ok(vec![])
}
fn after_list_act(res: Vec<serde_json::Value>) -> Result<Vec<Act>, anyhow::Error>{
    // 後処理
    Ok(vec![])
}

#[derive(Deserialize, Serialize, Arbitrary, Debug)]
struct Cast{}
async fn list_cast(req:()) -> Result<Vec<Cast>, anyhow::Error>{
    let a = pre_list_cast(req)?;
    let b = io_list_cast(a)?;
    let c = after_list_cast(b)?;
    Ok(c)
}
fn pre_list_cast(req: ()) -> Result<(), anyhow::Error>{
    // なんか引数チェック
    Ok(())
}
fn io_list_cast(req: ()) -> Result<Vec<serde_json::Value>, anyhow::Error>{
    // リトライとか
    Ok(vec![])
}

fn after_list_cast(res: Vec<serde_json::Value>) -> Result<Vec<Cast>, anyhow::Error>{
    // 後処理
    Ok(vec![])
}

