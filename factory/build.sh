RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
near deploy --wasmFile target/wasm32-unknown-unknown/release/factory.wasm --accountId factory.kula.testnet

