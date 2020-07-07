const express = require('express');
const path = require('path');
express.static.mime.define({'application/wasm': ['wasm']});
var app = express();

app.use('/', express.static(path.resolve(__dirname, "../dist")));

app.listen(process.env.PORT, function () {
  console.log('Example app listening on port 3000!')
})