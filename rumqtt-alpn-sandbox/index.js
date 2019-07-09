const mqtt = require("mqtt");
const fs = require("fs");
require('dotenv').config()
const client = mqtt.connect({
    // NOTE: mqtt の clientId には thingName を指定すること
    clientId: process.env.THING_NAME,
    ALPNProtocols: ["x-amzn-mqtt-ca"],
    protocol: "mqtts",
    port: 443,
    hostname: process.env.HOST,
    minVersion: "TLSv1.2",
    requestCert: true,
    checkServerIdentity: (servername, peer) => {
      console.log({ servername, peer });
    },
    // https://docs.aws.amazon.com/ja_jp/iot/latest/developerguide/managing-device-certs.html
    // RSA 2048 ビットキー: Amazon ルート CA 1 - https://www.amazontrust.com/repository/AmazonRootCA1.pem
    // openssl に含まれているっぽいので指定しなくても動くっぽい
    // ca:  Buffer.from(fs.readFileSync(process.env.ROOT_CA_PATH)),
    key: Buffer.from(fs.readFileSync(process.env.PRIVATE_KEY_PATH)),
    cert: Buffer.from(fs.readFileSync(process.env.CLIENT_CERT_PATH))
});
[
  "connect",
  "recoonect",
  "close",
  "offline",
  "error",
  "message",
  "packetsend",
  "packetreceive",
  "end"
].forEach((key) => {
  client.on(key, console.info.bind(console, key));
});
client.on("error", console.error.bind(console, "error"));
client.on("connect", ()=>{
    client.subscribe("legokichi/foo", console.log);
    setInterval(()=>{
        client.publish("legokichi/foo", "hello", console.log);
    }, 1000*10)
});
