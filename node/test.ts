const native = require("./index.node");

console.log("Exported keys:", Object.keys(native));

console.log("Available providers:", native.listProviders());

native
  .chat([
    { role: "system", content: "You are a Rust expert." },
    { role: "user", content: "Explain ownership in Rust." },
  ])
  .then((response: string) => {
    console.log("Chat response:", response);
  })
  .catch((error: unknown) => {
    console.error("Chat error:", error);
  });
