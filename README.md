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
- NodeJS (For codegen) <https://nodejs.org/en/download/>

## Development

### Setup

Setup kafka in docker containers and topics.

```bash
make init
```

Start producer & subscriber services

```bash
npx nf start
```

## Links

- <https://github.com/cloudevents/spec/blob/main/cloudevents/formats/cloudevents.proto>
- <https://docs.rs/protobuf-codegen/latest/protobuf_codegen/>
