require("http").createServer((req, res)=>{
    console.log("incomming");
    setTimeout(()=>{
        console.log("ok");
        res.writeHead(200, {"Content-Type": "text/plain"});
        res.write("Hello World");
        res.end();
    }, 0);
}).listen(8888);