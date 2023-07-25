const pkg = require("./pkg/fastly_vcl_with_rust")
const path = require('path');
const fs = require('fs');

const data = fs.readFileSync(path.resolve('src/backends.json'), 'utf-8')

console.log(pkg.build_backends(data))
