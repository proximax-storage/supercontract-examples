# Prerequisites
- rust
- wasm-pack

### Step 1: Update submodules

`git submodule update --init --recursive`

### Step 2: Copy contract source code to the `rust-xpx-supercontract-client-sdk/src/lib.rs`

### Step 3: Generate the binary (wasm file) required to deploy contract 
`wasm-pack build rust-xpx-supercontract-client-sdk --out-dir ../pkg`

### Step 4: The contract bytecode is located at `pkg/sdk_bg.wasm`

### Step 5: Deploy the `sdk_bg.wasm` using Sirius Chain Storage Tool