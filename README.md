## Compiling

- To compile the client `cargo-component` is needed to be installed on the system with rust target `wasm32-unknown-unknown`
- The command for compiling the client is: 

```bash
## This command should be run from the client directory 
cargo component build --target wasm32-unknown-unknown --release
```

- After that you can run the host from withing its directory normally with cargo 

```bash
cargo run -r
```

