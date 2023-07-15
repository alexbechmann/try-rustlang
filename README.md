# Try rustlang

## Prerequisites

- Docker
- [rustup](https://rustup.rs/)

  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

- Rust extension <https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer>
- Microsoft C++ <https://marketplace.visualstudio.com/items?itemName=ms-vscode.cpptools> (ms-vscode.cpptools) – on Windows
- CodeLLDB <https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb> (vadimcn.vscode-lldb) – on macOS/Linux

## Run kafka with docker

### Setup

Setup kafka in docker containers and topics.

```bash
make init
```

Start producer & subscriber services

```bash
npx nf start
```
