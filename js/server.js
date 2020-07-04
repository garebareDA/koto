const express = require('express');
const app = express();

// Set the MIME type explicitly
express.static.mime.define({'application/wasm': ['wasm']});

app.use(express.static('./dist'));

app.listen(8000);