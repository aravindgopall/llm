## 🚀 LLM SDK

Multi-provider, multi-language LLM wrapper with support for:

- ✅ **Rust** native use
- ✅ **Python** via [PyO3](https://pyo3.rs)
- ✅ **Node.js/TypeScript** via [napi-rs](https://napi.rs)

---

## 📁 Project Structure

```
llm/
├── rust/       # Core logic + all bindings
├── python/     # Python packaging (maturin)
├── node/       # Node.js addon (.node module)
```

---

## 🔧 Requirements

- Rust (>= 1.70)
- Node.js (>= 18)
- Python (>= 3.8)
- `maturin` (`pip install maturin`)
- `@napi-rs/cli` (`npm install -g @napi-rs/cli`)

---

## 🦀 Rust Usage

Add this to your external `Cargo.toml`:

```toml
llm = { git = "https://github.com/aravindgopall/llm", features = ["default"] }
```

Example:

```rust
use llm::config::Config;
use llm::provider::get_available_providers;

fn main() {
    let config = Config::from_env();
    let providers = get_available_providers(&config);
    for p in providers {
        println!("Available: {}", p.name());
    }
}
```

---

## 🐍 Python Usage

### ✅ Setup

```bash
cd python
maturin develop --features python-bindings
```

Or build a `.whl`:

```bash
maturin build --release --features python-bindings
```

### ✅ Example

```python
import llm

print(llm.list_providers())
print(llm.generate_response("Tell me about Rust."))
```

---

## 🟦 Node.js / TypeScript Usage

### ✅ Setup

```bash
cd node
npm install
napi build --release --cargo-cwd ../rust --features node-bindings
```

### ✅ Example (JavaScript)

```js
const { listProviders, generateResponse } = require("./index.node");

console.log(listProviders());
generateResponse("Explain ownership in Rust.").then(console.log);
```

### ✅ Example (TypeScript)

```ts
import { listProviders, generateResponse } from "./index.node";

console.log(listProviders());
generateResponse("Tell me a Rust joke.").then(console.log);
```

---

## 🧪 Testing

### Python

```bash
pytest
```

### Node

```bash
node tests/basic.js
```

---

## 🧪 Env Configuration

The SDK reads from `.env` or `process.env`:

```env
OPENAI_API_KEY=...
AZURE_OPENAI_KEY=...
AZURE_OPENAI_ENDPOINT=...
AZURE_DEPLOYMENT_ID=...
GEMINI_API_KEY=...
OLLAMA_MODEL=llama3
```

---

## 📦 Publishing

### Python

```bash
cd python
maturin build --release
twine upload target/wheels/*.whl
```

### Node.js

```bash
cd node
npm publish
```

---

## 🧠 Design Highlights

- Single Rust core for LLM abstraction
- Clean, optional bindings with `--features`
- Async-ready for both Python and Node
- Minimal duplication across languages
