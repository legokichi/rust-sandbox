## scylla について

* https://docs.scylladb.com/operating-scylla/procedures/best_practices_scylla_on_docker/

```bash
docker run --net=host --name scylla -d --rm -v $(pwd):/workspace scylladb/scylla
docker stop scylla
```

## cqlsh について

* https://docs.datastax.com/ja/cql-jajp/3.1/cql/cql_reference/cqlsh.html

```bash
docker exec -it scylla nodetool status
docker exec -it scylla cqlsh
docker exec -it scylla cqlsh -f /workspace/o.cql
```

## rust binding について

* https://github.com/AlexPikalov/cdrs
* https://docs.rs/cdrs/2.0.0-beta.1/cdrs/index.html



## CQL について
* https://docs.datastax.com/ja/cql-jajp/3.1/cql/cql_reference/cqlReferenceTOC.html
* https://gihyo.jp/dev/serial/01/cassandra/0003
* https://wiki.apache.org/cassandra/DataModel_JP

名前空間

* `［キースペース］［カラムファミリ］［キー］［カラム］`
* `［キースペース］［カラムファミリ］［キー］［スーパーカラム］［カラム］`

```ts
type Column = { name: String, value: Bytes, timestmap: UnixTime };
type SuperColumn = [{ key: String, columns: Column[] }];
type ColumnFamiry = [SuperColumn];

const profile: SuperColumn = {
    { key: "github", columns: [
        { name: "account", value: "legokichi@email.com", timestmap: 0 },
        { name: "password", value: "unchichang", timestmap: 0 },
    ] },
    { key: "hatena", columns: [
        { name: "account", value: "legokichi@email.com", timestmap: 0 },
        { name: "password", value: "unchichang", timestmap: 0 },
    ] }
}
```


CQL|SQL|役割
-------|----------|-----------
キースペース|データベース|レプリケーションの設定単位
カラムファミリ|テーブル|今はテーブルと呼ばれているっぽい？

```console
cqlsh> DESCRIBE keyspaces;
cqpsh> DESCRIBE keyspace keyspace_name;
cqlsh> DESCRIBE tables;
cqlsh> CREATE KEYSPACE my_app
WITH replication = {'class': 'SimpleStrategy', 'replication_factor': 1;
cqlsh> USE keyspace_name;
cqlsh:my_app> CREATE TABLE user (
  user_id varchar PRIMARY KEY,
  first_name varchar,
  last_name varchar
);

```
