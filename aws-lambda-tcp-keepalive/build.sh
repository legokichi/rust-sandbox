cargo +nightly build --release 
cp ./target/release/aws-lambda-tcp-keepalive ./bootstrap
zip -r lambda.zip bootstrap
rm bootstrap
