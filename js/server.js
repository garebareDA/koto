const express = require('express');
express.static.mime.define({'application/wasm': ['wasm']})
var app = express();

app.use('/', express.static('../dist'));


app.listen(8000, function () {
  console.log('Example app listening on port 8000!')
})