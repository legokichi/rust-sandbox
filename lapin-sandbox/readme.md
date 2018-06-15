# lapin-futures's Heisenbugs

## 1st, run rabbitmq server in docker

```sh
docker run  \
    -e RABBITMQ_ERLANG_COOKIE="SWQOKODSQALRPCLNMEQG" \
    -e RABBITMQ_DEFAULT_USER="rabbitmq" \
    -e RABBITMQ_DEFAULT_PASS="rabbitmq" \
    -e RABBITMQ_DEFAULT_VHOST="/" \
    -p 15672:15672 \
    -p 5672:5672 \
    rabbitmq:3.7-management
```

## 2nd, run this crate

### `RUST_LOG=lapin_futures=debug`

```console
$ env RUST_LOG=lapin_futures=debug RUST_BACKTRACE=1 cargo run
   Compiling lapin-sandbox v0.1.0 (file:///home/legokichi/Github/rust-snipets/lapin-sandbox)
    Finished dev [unoptimized + debuginfo] target(s) in 3.84 secs
     Running `target/debug/lapin-sandbox`
DEBUG 2018-06-15T10:00:47Z: lapin_futures::transport: AMQPTransportConnector poll transport is none? false
DEBUG 2018-06-15T10:00:47Z: lapin_futures::transport: conn state: Connecting(SentProtocolHeader)
DEBUG 2018-06-15T10:00:47Z: lapin_futures::transport: AMQPTransportConnector poll transport is none? false
DEBUG 2018-06-15T10:00:47Z: lapin_futures::transport: conn state: Connecting(SentProtocolHeader)
DEBUG 2018-06-15T10:00:47Z: lapin_futures::transport: AMQPTransportConnector poll transport is none? false
DEBUG 2018-06-15T10:00:47Z: lapin_futures::transport: conn state: Connecting(SentProtocolHeader)
DEBUG 2018-06-15T10:00:47Z: lapin_futures::transport: AMQPTransportConnector poll transport is none? false
DEBUG 2018-06-15T10:00:47Z: lapin_futures::transport: conn state: Connecting(SentStartOk)
DEBUG 2018-06-15T10:00:47Z: lapin_futures::transport: AMQPTransportConnector poll transport is none? false
DEBUG 2018-06-15T10:00:47Z: lapin_futures::transport: conn state: Connecting(SentOpen)
DEBUG 2018-06-15T10:00:47Z: lapin_futures::transport: AMQPTransportConnector poll transport is none? false
DEBUG 2018-06-15T10:00:47Z: lapin_futures::transport: conn state: Connected
DEBUG 2018-06-15T10:00:47Z: lapin_futures::transport: already connected
DEBUG 2018-06-15T10:00:47Z: lapin_futures::client: got client service
ready
published
Ok(NotReady)
Ok(NotReady)
Ok(NotReady)
Ok(NotReady)
Ok(NotReady)
Ok(NotReady)
Ok(NotReady)
Ok(NotReady)
Ok(NotReady)
Ok(NotReady)
Ok(NotReady)
Ok(NotReady)
Ok(NotReady)
Ok(NotReady)
Ok(NotReady)
Ok(NotReady)
Ok(NotReady)
Ok(NotReady)
Ok(NotReady)
Ok(NotReady)
Ok(NotReady)
Ok(NotReady)
Ok(NotReady)
Ok(NotReady)
Ok(NotReady)
Ok(NotReady)
Ok(NotReady)
Ok(NotReady)
Ok(NotReady)
Ok(NotReady)
Ok(NotReady)
Ok(NotReady)
Ok(NotReady)
Ok(NotReady)
Ok(NotReady)
Ok(NotReady)
Ok(NotReady)
Ok(NotReady)
Ok(NotReady)
Ok(NotReady)
Ok(NotReady)
Ok(NotReady)
Ok(NotReady)
Err(Custom { kind: Other, error: StringError("basic get returned empty") })
```

suddenly stopeed.

### `RUST_LOG=lapin_futures=trace`

it works.

```console
$ env RUST_LOG=lapin_futures=trace RUST_BACKTRACE=1 cargo run
   Compiling lapin-sandbox v0.1.0 (file:///home/legokichi/Github/rust-snipets/lapin-sandbox)
    Finished dev [unoptimized + debuginfo] target(s) in 4.58 secs
     Running `target/debug/lapin-sandbox`
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: send frames
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: sink start send
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: will encode and write frame: ProtocolHeader
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: serialized frame: 8 bytes
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: sink poll_complete
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: handle frames
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: stream poll
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: upstream poll gave NotReady
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: pre-poll
DEBUG 2018-06-15T10:01:39Z: lapin_futures::transport: AMQPTransportConnector poll transport is none? false
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: send frames
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: handle frames
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: stream poll
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: upstream poll gave NotReady
DEBUG 2018-06-15T10:01:39Z: lapin_futures::transport: conn state: Connecting(SentProtocolHeader)
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: waiting before poll
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: stream poll
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: upstream poll gave NotReady
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: post-poll
DEBUG 2018-06-15T10:01:39Z: lapin_futures::transport: AMQPTransportConnector poll transport is none? false
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: send frames
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: handle frames
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: stream poll
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: upstream poll gave NotReady
DEBUG 2018-06-15T10:01:39Z: lapin_futures::transport: conn state: Connecting(SentProtocolHeader)
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: waiting before poll
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: stream poll
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: upstream poll gave NotReady
DEBUG 2018-06-15T10:01:39Z: lapin_futures::transport: AMQPTransportConnector poll transport is none? false
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: send frames
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: handle frames
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: stream poll
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: decoded frame: Method(0, Connection(Start(Start { version_major: 0, version_minor: 9, server_properties: {"capabilities": FieldTable({"authentication_failure_close": Boolean(true), "basic.nack": Boolean(true), "connection.blocked": Boolean(true), "consumer_cancel_notify": Boolean(true), "consumer_priorities": Boolean(true), "direct_reply_to": Boolean(true), "exchange_exchange_bindings": Boolean(true), "per_consumer_qos": Boolean(true), "publisher_confirms": Boolean(true)}), "cluster_name": LongString("rabbit@78b4754f68c2"), "copyright": LongString("Copyright (C) 2007-2018 Pivotal Software, Inc."), "information": LongString("Licensed under the MPL.  See http://www.rabbitmq.com/"), "platform": LongString("Erlang/OTP 20.3.5"), "product": LongString("RabbitMQ"), "version": LongString("3.7.5")}, mechanisms: "AMQPLAIN PLAIN", locales: "en_US" })))
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: upstream poll gave frame: Method(0, Connection(Start(Start { version_major: 0, version_minor: 9, server_properties: {"capabilities": FieldTable({"authentication_failure_close": Boolean(true), "basic.nack": Boolean(true), "connection.blocked": Boolean(true), "consumer_cancel_notify": Boolean(true), "consumer_priorities": Boolean(true), "direct_reply_to": Boolean(true), "exchange_exchange_bindings": Boolean(true), "per_consumer_qos": Boolean(true), "publisher_confirms": Boolean(true)}), "cluster_name": LongString("rabbit@78b4754f68c2"), "copyright": LongString("Copyright (C) 2007-2018 Pivotal Software, Inc."), "information": LongString("Licensed under the MPL.  See http://www.rabbitmq.com/"), "platform": LongString("Erlang/OTP 20.3.5"), "product": LongString("RabbitMQ"), "version": LongString("3.7.5")}, mechanisms: "AMQPLAIN PLAIN", locales: "en_US" })))
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: send frames
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: sink start send
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: will encode and write frame: Method(0, Connection(StartOk(StartOk { client_properties: {"product": LongString("lapin")}, mechanism: "PLAIN", response: "\u{0}rabbitmq\u{0}rabbitmq", locale: "en_US" })))
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: serialized frame: 68 bytes
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: sink poll_complete
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: stream poll
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: upstream poll gave NotReady
DEBUG 2018-06-15T10:01:39Z: lapin_futures::transport: conn state: Connecting(SentStartOk)
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: waiting before poll
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: stream poll
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: upstream poll gave NotReady
DEBUG 2018-06-15T10:01:39Z: lapin_futures::transport: AMQPTransportConnector poll transport is none? false
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: send frames
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: handle frames
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: stream poll
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: decoded frame: Method(0, Connection(Tune(Tune { channel_max: 2047, frame_max: 131072, heartbeat: 60 })))
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: upstream poll gave frame: Method(0, Connection(Tune(Tune { channel_max: 2047, frame_max: 131072, heartbeat: 60 })))
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: send frames
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: sink start send
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: will encode and write frame: Method(0, Connection(TuneOk(TuneOk { channel_max: 2047, frame_max: 131072, heartbeat: 60 })))
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: serialized frame: 20 bytes
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: sink poll_complete
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: sink start send
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: will encode and write frame: Method(0, Connection(Open(Open { virtual_host: "/", capabilities: "", insist: false })))
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: serialized frame: 16 bytes
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: sink poll_complete
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: stream poll
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: upstream poll gave NotReady
DEBUG 2018-06-15T10:01:39Z: lapin_futures::transport: conn state: Connecting(SentOpen)
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: waiting before poll
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: stream poll
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: upstream poll gave NotReady
DEBUG 2018-06-15T10:01:39Z: lapin_futures::transport: AMQPTransportConnector poll transport is none? false
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: send frames
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: handle frames
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: stream poll
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: decoded frame: Method(0, Connection(OpenOk(OpenOk { known_hosts: "" })))
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: upstream poll gave frame: Method(0, Connection(OpenOk(OpenOk { known_hosts: "" })))
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: send frames
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: stream poll
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: upstream poll gave NotReady
DEBUG 2018-06-15T10:01:39Z: lapin_futures::transport: conn state: Connected
DEBUG 2018-06-15T10:01:39Z: lapin_futures::transport: already connected
DEBUG 2018-06-15T10:01:39Z: lapin_futures::client: got client service
TRACE 2018-06-15T10:01:39Z: lapin_futures::channel: create request id: Some(0)
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: send frames
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: sink start send
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: will encode and write frame: Method(1, Channel(Open(Open { out_of_band: "" })))
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: serialized frame: 13 bytes
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: sink poll_complete
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: handle frames
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: stream poll
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: upstream poll gave NotReady
TRACE 2018-06-15T10:01:39Z: lapin_futures::channel: create returning closure
TRACE 2018-06-15T10:01:39Z: lapin_futures::channel: wait for answer for request 0
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: send frames
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: handle frames
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: stream poll
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: upstream poll gave NotReady
TRACE 2018-06-15T10:01:39Z: lapin_futures::channel: create request id: Some(1)
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: send frames
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: sink start send
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: will encode and write frame: Method(2, Channel(Open(Open { out_of_band: "" })))
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: serialized frame: 13 bytes
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: sink poll_complete
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: handle frames
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: stream poll
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: decoded frame: Method(1, Channel(OpenOk(OpenOk { channel_id: "" })))
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: upstream poll gave frame: Method(1, Channel(OpenOk(OpenOk { channel_id: "" })))
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: send frames
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: stream poll
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: upstream poll gave NotReady
TRACE 2018-06-15T10:01:39Z: lapin_futures::channel: create returning closure
TRACE 2018-06-15T10:01:39Z: lapin_futures::channel: wait for answer for request 1
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: send frames
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: handle frames
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: stream poll
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: upstream poll gave NotReady
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: send frames
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: handle frames
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: stream poll
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: upstream poll gave NotReady
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: send frames
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: handle frames
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: stream poll
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: upstream poll gave NotReady
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: send frames
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: handle frames
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: stream poll
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: decoded frame: Method(2, Channel(OpenOk(OpenOk { channel_id: "" })))
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: upstream poll gave frame: Method(2, Channel(OpenOk(OpenOk { channel_id: "" })))
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: send frames
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: stream poll
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: upstream poll gave NotReady
ready
TRACE 2018-06-15T10:01:39Z: lapin_futures::channel: queue_declare request id: Some(2)
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: send frames
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: sink start send
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: will encode and write frame: Method(1, Queue(Declare(Declare { ticket: 0, queue: "foo", passive: false, durable: false, exclusive: false, auto_delete: false, nowait: false, arguments: {} })))
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: serialized frame: 23 bytes
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: sink poll_complete
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: handle frames
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: stream poll
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: upstream poll gave NotReady
TRACE 2018-06-15T10:01:39Z: lapin_futures::channel: queue_declare returning closure
TRACE 2018-06-15T10:01:39Z: lapin_futures::channel: wait for answer for request 2
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: send frames
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: handle frames
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: stream poll
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: upstream poll gave NotReady
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: send frames
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: handle frames
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: stream poll
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: decoded frame: Method(1, Queue(DeclareOk(DeclareOk { queue: "foo", message_count: 2, consumer_count: 0 })))
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: upstream poll gave frame: Method(1, Queue(DeclareOk(DeclareOk { queue: "foo", message_count: 2, consumer_count: 0 })))
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: send frames
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: stream poll
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: upstream poll gave NotReady
TRACE 2018-06-15T10:01:39Z: lapin_futures::channel: basic_publish request id: Some(0)
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: send frames
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: sink start send
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: will encode and write frame: Method(1, Basic(Publish(Publish { ticket: 0, exchange: "", routing_key: "foo", mandatory: false, immediate: false })))
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: serialized frame: 20 bytes
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: sink poll_complete
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: handle frames
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: stream poll
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: upstream poll gave NotReady
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: send frames
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: sink start send
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: will encode and write frame: Header(1, 60, ContentHeader { class_id: 60, weight: 0, body_size: 2, properties: Properties { content_type: None, content_encoding: None, headers: None, delivery_mode: None, priority: None, correlation_id: None, reply_to: None, expiration: None, message_id: None, timestamp: None, type_: None, user_id: None, app_id: None, cluster_id: None } })
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: serialized frame: 22 bytes
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: sink poll_complete
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: sink start send
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: will encode and write frame: Body(1, [104, 105])
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: serialized frame: 10 bytes
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: sink poll_complete
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: handle frames
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: stream poll
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: upstream poll gave NotReady
TRACE 2018-06-15T10:01:39Z: lapin_futures::channel: basic_publish returning closure
TRACE 2018-06-15T10:01:39Z: lapin_futures::channel: wait for answer for request 0
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: send frames
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: handle frames
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: stream poll
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: upstream poll gave NotReady
published
TRACE 2018-06-15T10:01:39Z: lapin_futures::channel: queue_declare request id: Some(4)
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: send frames
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: sink start send
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: will encode and write frame: Method(2, Queue(Declare(Declare { ticket: 0, queue: "foo", passive: false, durable: false, exclusive: false, auto_delete: false, nowait: false, arguments: {} })))
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: serialized frame: 23 bytes
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: sink poll_complete
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: handle frames
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: stream poll
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: upstream poll gave NotReady
TRACE 2018-06-15T10:01:39Z: lapin_futures::channel: queue_declare returning closure
TRACE 2018-06-15T10:01:39Z: lapin_futures::channel: wait for answer for request 4
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: send frames
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: handle frames
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: stream poll
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: upstream poll gave NotReady
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: send frames
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: handle frames
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: stream poll
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: decoded frame: Method(2, Queue(DeclareOk(DeclareOk { queue: "foo", message_count: 2, consumer_count: 0 })))
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: upstream poll gave frame: Method(2, Queue(DeclareOk(DeclareOk { queue: "foo", message_count: 2, consumer_count: 0 })))
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: send frames
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: stream poll
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: upstream poll gave NotReady
TRACE 2018-06-15T10:01:39Z: lapin_futures::channel: basic_get request id: Some(5)
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: send frames
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: sink start send
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: will encode and write frame: Method(2, Basic(Get(Get { ticket: 0, queue: "foo", no_ack: false })))
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: serialized frame: 19 bytes
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: sink poll_complete
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: handle frames
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: stream poll
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: upstream poll gave NotReady
TRACE 2018-06-15T10:01:39Z: lapin_futures::channel: basic_get returning closure
TRACE 2018-06-15T10:01:39Z: lapin_futures::channel: wait for answer for request 5
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: send frames
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: handle frames
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: stream poll
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: upstream poll gave NotReady
Ok(NotReady)
TRACE 2018-06-15T10:01:39Z: lapin_futures::channel: basic_get request id: Some(6)
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: send frames
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: sink start send
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: will encode and write frame: Method(2, Basic(Get(Get { ticket: 0, queue: "foo", no_ack: false })))
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: serialized frame: 19 bytes
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: sink poll_complete
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: handle frames
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: stream poll
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: decoded frame: Method(2, Basic(GetOk(GetOk { delivery_tag: 1, redelivered: true, exchange: "", routing_key: "foo", message_count: 2 })))
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: upstream poll gave frame: Method(2, Basic(GetOk(GetOk { delivery_tag: 1, redelivered: true, exchange: "", routing_key: "foo", message_count: 2 })))
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: send frames
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: stream poll
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: decoded frame: Header(2, 60, ContentHeader { class_id: 60, weight: 0, body_size: 2, properties: Properties { content_type: None, content_encoding: None, headers: None, delivery_mode: None, priority: None, correlation_id: None, reply_to: None, expiration: None, message_id: None, timestamp: None, type_: None, user_id: None, app_id: None, cluster_id: None } })
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: upstream poll gave frame: Header(2, 60, ContentHeader { class_id: 60, weight: 0, body_size: 2, properties: Properties { content_type: None, content_encoding: None, headers: None, delivery_mode: None, priority: None, correlation_id: None, reply_to: None, expiration: None, message_id: None, timestamp: None, type_: None, user_id: None, app_id: None, cluster_id: None } })
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: send frames
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: stream poll
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: decoded frame: Body(2, [104, 105])
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: upstream poll gave frame: Body(2, [104, 105])
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: send frames
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: stream poll
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: decoded frame: Method(2, Basic(GetOk(GetOk { delivery_tag: 2, redelivered: true, exchange: "", routing_key: "foo", message_count: 1 })))
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: upstream poll gave frame: Method(2, Basic(GetOk(GetOk { delivery_tag: 2, redelivered: true, exchange: "", routing_key: "foo", message_count: 1 })))
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: send frames
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: stream poll
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: decoded frame: Header(2, 60, ContentHeader { class_id: 60, weight: 0, body_size: 2, properties: Properties { content_type: None, content_encoding: None, headers: None, delivery_mode: None, priority: None, correlation_id: None, reply_to: None, expiration: None, message_id: None, timestamp: None, type_: None, user_id: None, app_id: None, cluster_id: None } })
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: upstream poll gave frame: Header(2, 60, ContentHeader { class_id: 60, weight: 0, body_size: 2, properties: Properties { content_type: None, content_encoding: None, headers: None, delivery_mode: None, priority: None, correlation_id: None, reply_to: None, expiration: None, message_id: None, timestamp: None, type_: None, user_id: None, app_id: None, cluster_id: None } })
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: send frames
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: stream poll
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: decoded frame: Body(2, [104, 105])
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: upstream poll gave frame: Body(2, [104, 105])
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: send frames
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: stream poll
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: upstream poll gave NotReady
TRACE 2018-06-15T10:01:39Z: lapin_futures::channel: basic_get returning closure
TRACE 2018-06-15T10:01:39Z: lapin_futures::channel: wait for answer for request 6
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: send frames
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: handle frames
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: stream poll
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: upstream poll gave NotReady
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: send frames
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: handle frames
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: stream poll
TRACE 2018-06-15T10:01:39Z: lapin_futures::transport: upstream poll gave NotReady
Ok(Ready(BasicGetMessage { delivery: Delivery { delivery_tag: 1, exchange: "", routing_key: "foo", redelivered: true, properties: Properties { content_type: None, content_encoding: None, headers: None, delivery_mode: None, priority: None, correlation_id: None, reply_to: None, expiration: None, message_id: None, timestamp: None, type_: None, user_id: None, app_id: None, cluster_id: None }, data: [104, 105] }, message_count: 2 }))
hi
end
```