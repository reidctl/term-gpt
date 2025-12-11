# 🧠 gpt — A ChatGPT-powered CLI for Your Terminal

**`term-gpt`** is a fast, colorful, file-aware ChatGPT client for your terminal.  
It supports one-shot prompts, REPL mode, file context, and a custom assistant personality.

Use it like:

```bash
gpt "Explain Rust ownership like I'm an exhausted developer."
```

Or enter interactive chat:

```bash
gpt --repl
```

Supports injecting files into the prompt:

```bash
gpt -f src/main.rs "Explain what this code does."
```

---

## ✨ Features

- 🔥 **One-shot prompts**
- 💬 **Interactive REPL mode** (`--repl`)
- 📄 **Include files as context** (`-f path/to/file`)
- 🎨 **Colored output** (user blue, assistant green)
- 🧠 **Custom assistant personality** baked into the tool
- 🔐 **Uses your own OpenAI API key**
- ⚙️ **Configurable, simple, and fast**

---

## 🚀 Installation

### 1. Install 

```bash
cargo install --git https://github.com/YOURNAME/gpt
```

Or install through cargo:

```bash
cargo install term-gpt
```

### 2. Ensure Cargo bin is in your PATH

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

### 3. Set your OpenAI API key

```bash
export OPENAI_API_KEY="sk-your-key-here"
```

> ⚠️ Never hardcode your API key into source files.  
> ⚠️ Never commit `.env` files with secrets.

---

## 🧩 Usage

### One-shot prompt

```bash
gpt "Write me a haiku about Rust and caffeine dependency."
```

### Provide files as context

```bash
gpt -f src/main.rs "Explain this code."
```

Multiple files:

```bash
gpt -f src/main.rs -f README.md "Summarize the project."
```

### REPL mode

```bash
gpt --repl
```

Quit with:

```
:q
```

---

## 🖍 Example Output

```
You > Why is Rust so strict?

Assistant >
Because Rust is the gym coach who screams “FORM MATTERS”
while making sure you don’t blow out your back doing a deadlift
with a null pointer.
```

---

## 🛠 Building From Source

```bash
git clone https://github.com/reid-ctl/term-gpt
cd gpt
cargo build --release
```

Optimized binary:

```
target/release/gpt
```

Symlink:

```bash
sudo ln -s $(pwd)/target/release/gpt /usr/local/bin/gpt
```

---

## ⚙️ Configuration

This CLI uses OpenAI’s Responses API.

You can tweak:

- model  
- assistant personality  
- formatting/colors  
- REPL behavior  

Inside `DEFAULT_PERSONALITY` in `main.rs`.

---

## 🧪 To-Do / Future Enhancements

- [ ] Streaming responses  
- [ ] `--model` flag  
- [ ] Save chat transcripts  
- [ ] Pipe stdin (`cat file | gpt`)  
- [ ] Shell completions  
- [ ] Arch package (`PKGBUILD`)  

---

## 📜 License

MIT — do whatever you want, just don’t sue me if you anger the borrow checker.

---

## 💬 Author

Made by Garrett Reid,  
powered by Rust, caffeine, and an unhealthy relationship with terminal prompts.
