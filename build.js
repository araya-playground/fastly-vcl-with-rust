const pkg = require("./pkg/fastly_vcl_with_rust")

console.log(pkg.build_backends(`[{"name": "hello"}]`))
