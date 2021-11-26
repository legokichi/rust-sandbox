ompiling rust-lambda-panic v0.1.0 (/home/legokichi/Github/rust-lambda-panic)
warning: unreachable expression
  --> src/main.rs:13:5
   |
12 |     panic!("hoge");
   |     -------------- any code following this expression is unreachable
13 |     Ok(serde_json::json!({}))
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^ unreachable expression
   |
   = note: `#[warn(unreachable_code)]` on by default

warning: unused variable: `event`
  --> src/main.rs:10:15
   |
10 | async fn func(event: serde_json::Value, _: lambda_runtime::Context) -> Result<serde_json::Value, lambda_runtime::Error> {
   |               ^^^^^ help: if this is intentional, prefix it with an underscore: `_event`
   |
   = note: `#[warn(unused_variables)]` on by default

warning: `rust-lambda-panic` (bin "rust-lambda-panic") generated 2 warnings
    Finished dev [unoptimized + debuginfo] target(s) in 1.78s
legokichi@duxca:/home/legokichi/Github/rust-lambda-panic git:(master*) $ cargo build
legokichi@duxca:/home/legokichi/Github/rust-lambda-panic git:(master*) $ mv
legokichi@duxca:/home/legokichi/Github/rust-lambda-panic git:(master*) $ mv target/debug/rust-lambda-panic ./bootstrap
mv: overwrite './bootstrap'? y
legokichi@duxca:/home/legokichi/Github/rust-lambda-panic git:(master*) $ zip bootstrap.zip bootstrap
updating: bootstrap (deflated 77%)
legokichi@duxca:/home/legokichi/Github/rust-lambda-panic git:(master*) $ docker run \
    -i -e DOCKER_LAMBDA_USE_STDIN=1 \
    --rm \
    -v /home/legokichi/Github/rust-lambda-panic/target/debug/rust-lambda-panic:/var/task/bootstrap \
    lambci/lambda:provided.al2
START RequestId: b6eaa27b-e88a-1076-e331-11fa8a6a24db Version: $LATEST
END RequestId: b6eaa27b-e88a-1076-e331-11fa8a6a24db
REPORT RequestId: b6eaa27b-e88a-1076-e331-11fa8a6a24db  Init Duration: 1943.50 ms       Duration: 1.07 ms       Billed Duration: 2 ms   Memory Size: 1536 MB        Max Memory Used: 10 MB

{"errorType":"exitError","errorMessage":"RequestId: b6eaa27b-e88a-1076-e331-11fa8a6a24db Error: Couldn't find valid bootstrap(s): [/var/task/bootstrap /opt/bootstrap]"}

