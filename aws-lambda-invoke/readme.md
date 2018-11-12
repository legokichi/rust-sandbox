
```bash
aws lambda invoke \
    --invocation-type RequestResponse \
    --function-name="arn:aws:lambda:ap-northeast-1:xxxxx:function:xxxxx" \
    --payload='{"url":"https://example.com","method":"get"}' \
    --log-type=Tail \
    outfile
```

```
echo **** | base64 -d
```

aws-sdk-js

```json
POST "/2015-03-31/functions/arn%3Aaws%3Alambda%3Aap-northeast-1%3Axxxxxxx%3Afunction%3Axxxxx/invocations"
{
  "User-Agent": "aws-sdk-nodejs/2.335.0 linux/v10.11.0 callback",
  "X-Amz-Invocation-Type": "RequestResponse",
  "Content-Type": "binary/octet-stream",
  "X-Amz-Content-Sha256": "982b86ab5448c5eb2bf5e3fc8f5ae82aae37d5a491f61625aca4bc16914cebc0",
  "Content-Length": 44,
  "Host": "lambda.ap-northeast-1.amazonaws.com",
  "X-Amz-Date": "20181016T073337Z",
  "Authorization": "AWS4-HMAC-SHA256 Credential=AKIAI3OBNMU4PXZ5FNTQ/20181016/ap-northeast-1/lambda/aws4_request, SignedHeaders=host;x-amz-content-sha256;x-amz-date;x-amz-invocation-type, Signature=891d7b2a9b0cd4661c6f0c1687c5133bcd4ccd1077cff760ed55347e60738ea8"
}"
```

https://lambda.ap-northeast-1.amazonaws.com/2015-03-31/functions/arn%3Aaws%3Alambda%3Aap-northeast-1%3Axxxxxx%3Afunction%3Axxxxxx-xxxxxx-xxxxxx-webhook-sender/invocations





curl --request POST https://lambda.ap-northeast-1.amazonaws.com/2015-03-31/functions/arn:aws:lambda:ap-northeast-1:xxxxxx:function:xxxxxx-xxxxxx-xxxxxx-webhook-sender/invocations \
  --header "authorization: AWS4-HMAC-SHA256 Credential=AKIAI3OBNMU4PXZ5FNTQ/20181016/ap-northeast-1/lambda/aws4_request, SignedHeaders=host;x-amz-content-sha256;x-amz-date;x-amz-invocation-type, Signature=f41f6150b7359cae53f30a49fed132ec64cbd2c2a2bcf62467dd35877ed2be22" \
  --header "x-amz-invocation-type: RequestResponse" \
  --header "host: lambda.ap-northeast-1.amazonaws.com" \
  --header "x-amz-content-sha256: c3f2deac49d75e5c848f509b1ab0807736b121c3ea7f34f2b5fb94ba327d2290" \
  --header "x-amz-date: 20181016T110849Z" \
  --data-raw '{"method":"get","url":"https://example.com"}'