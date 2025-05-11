const native = require("../index.node");

console.log("Exported keys:", Object.keys(native));

console.log("Available providers:", native.list_providers());

native
  .generate_response("What is Rust?")
  .then((response) => console.log("Response:", response))
  .catch((err) => console.error("Error:", err));
