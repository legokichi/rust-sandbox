const aws = require("aws-sdk");

let url;
url = new aws.S3().getSignedUrl("putObject", {Bucket: 'bucket', Key: 'key'});
console.log(url);
url = new aws.S3().getSignedUrl("putObject", {Bucket: 'bucket', Key: 'key',
    Expires: 120, ACL: "bucket-owner-full-control", ContentType: "text/csv"});
console.log(url);
