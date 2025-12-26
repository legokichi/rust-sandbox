#!/bin/bash

# ストリームARNを設定
STREAM_ARN="arn:aws:kinesis:ap-northeast-1:242817538175:stream/device-api-dev-permanent-table-changes"

# タイムスタンプを設定 (例: 2023-01-01T00:00:00.000Z)
TIMESTAMP="2024-06-14T00:00:00.000Z"

# ストリームのシャードIDを動的に取得
SHARD_IDS=$(aws kinesis describe-stream --stream-arn $STREAM_ARN --query 'StreamDescription.Shards[*].ShardId' --output text)

# シャードIDごとにレコードを取得
for SHARD_ID in $SHARD_IDS; do
    # シャードのIteratorを取得
    SHARD_ITERATOR=$(aws kinesis get-shard-iterator \
        --stream-arn $STREAM_ARN \
        --shard-id $SHARD_ID \
        --shard-iterator-type AT_TIMESTAMP \
        --timestamp $TIMESTAMP \
        --query 'ShardIterator' \
        --output text)

    # レコードを取得
    while :
    do
        # レコードを取得
        OUTPUT=$(aws kinesis get-records \
            --shard-iterator $SHARD_ITERATOR \
            --query 'Records[*].Data' \
            --output text)

        if [ "$OUTPUT" != "" ]; then
            echo "Records from shard $SHARD_ID: $OUTPUT"
        else
            echo "No new records in shard $SHARD_ID."
        fi

        # 次のシャードのIteratorを取得
        SHARD_ITERATOR=$(aws kinesis get-records \
            --shard-iterator $SHARD_ITERATOR \
            --query 'NextShardIterator' \
            --output text)

        # 適宜スリープ
        sleep 5
    done &
done

# 全てのバックグラウンドジョブを待機
wait
