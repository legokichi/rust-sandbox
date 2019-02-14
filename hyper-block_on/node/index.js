const express = require("express");
const app = express();
app.get("/", (req, res)=>{
  setInterval(()=>{
    res.write("hello\r\n");
  }, 1000);
});
app.listen(8080);

