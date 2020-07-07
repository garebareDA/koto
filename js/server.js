const express = require('express');
express.static.mime.define({'application/wasm': ['wasm']})
var app = express();

app.use('/', express.static(__dirname + '/dist'));


app.listen(process.env.PORT, function () {
  console.log('Example app listening on port 8000!')
})