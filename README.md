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

## Preparation

- Install [ms-vscode-remote.remote-containers](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers) extension
- **If using SSH to clone**: Add ssh key to SSH agent <https://code.visualstudio.com/remote/advancedcontainers/sharing-git-credentials>
  ```
  ssh-add ~/.ssh/id_rsa
  ```
- Open repo in vscode devcontainer with:

  - CTRL+SHIFT+P -> Dev-Containers: Clone Repository in Named Volume
  - Paste repo HTTPS or SSH url
  - Choose volume name
  - Open terminal inside vscode to get a shell inside devcontainer

## Development

### Start producer & subscriber services

```bash
nf start
```

### Test

```bash
make test
```

## Links

- <https://github.com/cloudevents/spec/blob/main/cloudevents/formats/cloudevents.proto>
- <https://docs.rs/protobuf-codegen/latest/protobuf_codegen/>
- <https://docs.rs/shaku/0.6.1/shaku/guide/index.html>
