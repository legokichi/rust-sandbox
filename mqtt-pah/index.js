const mqtt = require('mqtt')
const aedes = require('aedes')()
const server = require('net').createServer(aedes.handle)
const port = 1883

const evs = [
  "client",
  "clientReady",
  "clientDisconnect",
  "clientError",
  "connectionError",
  "keepaliveTimeout",
  "publish",
  "ack",
  "ping",
  "subscribe",
  "unsubscribe",
  "connackSent",
  "closed",
  //
  "error",
  "ready",
  "new",
];
for(const ev of evs){
  aedes.on(ev, (...args)=>{
    args.pop();
    console.info.bind(console, "aedes", ev, ...args)
  });
}
aedes.on("client", (client)=>{
  console.info("aedes","client");
  const evs = [
    "connected",
    "error",
  ];
  for(const ev of evs){
    client.on(ev, console.info.bind(console, "aedes client", ev));
  }
});

server.listen(port, function () {
  console.log('server started and listening on port ', port)
  //const mclient = mqtt.connect(`mqtt://127.0.0.1:${port}`);
  //const evs = [
  //  "connect",
  //  "reconnect",
  //  "close",
  //  "disconnect",
  //  "offline",
  //  "error",
  //  "end",
  //  "message",
  //  "packetsend",
  //  "packetreceive",
  //];
  //for(ev of evs){
  //  mclient.on(ev, console.info.bind(console, "mqtt", ev));
  //}

  aedes.on("clientReady", (client)=>{
    console.log("aedes clientReady");
    setTimeout(()=>{
      console.log("aedes client closing");
      client.close(()=>{
        console.log("aedes client closed");
      });
    }, 6000);
  });
});
