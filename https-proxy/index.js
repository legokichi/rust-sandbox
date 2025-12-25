const ProxyServer = require('transparent-proxy');
const axios = require('axios');
const https = require('https');

const server = new ProxyServer({
    verbose: true,
    //intercept: true,
    //keys: (_ses) => ({
    //    key: fs.readFileSync('/usr/share/ca-certificates/local/local.key'),
    //    cert: fs.readFileSync('/usr/share/ca-certificates/local/local.crt')
    //}),
    //injectData: (data, ses) => {
    //    //console.log(data.toString("utf8"));
    //    console.log(Object.prototype.toString.call(data));
    //    return data;
    //},
    //injectResponse: (data, ses) => {
    //    let a = data.toString("utf8");
    //    console.log(Object.prototype.toString.call(data));
    //    a = a.replace("xample", "xenple");
    //    const b = Uint8Array.from(Buffer.from(a, "utf8"));
    //    return data;
    //}
});
const httpsAgent = new https.Agent({
    rejectUnauthorized: true,
});
server.on("error", (err) => {
    console.log(err);
});
server.listen(8443, '0.0.0.0', function() {
    console.log('TCP-Proxy-Server started!', server.address());
    //return;
    setTimeout(() => {
        axios.get("https://exmaple.com/", {
            httpsAgent,
            proxy: {
                protocol: 'http',
                host: "127.0.0.1",
                port: 8443,
            }
        }).then((res) => {
            console.log(res.data);
        }
        ).catch((err) => {
            console.log(err);
        });
    }, 1000);
});
