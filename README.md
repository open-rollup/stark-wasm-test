# Stark-Wasm-Test

Running [winterfell](https://github.com/novifinancial/winterfell) and [miden](https://github.com/maticnetwork/miden) with [Wasmtime](https://docs.wasmtime.dev/introduction.html)

## Running natively

```shell
cargo run --release
```

## Running with Wasmtime

```shell
cargo build --target wasm32-wasi --no-default-features --release
wasmtime target/wasm32-wasi/release/stark-wasm.wasm
```

## Result

Computing Fibonacci sequence up to 65536th term, Proof security: 96 bits (CPU: i5-1135G7)

|                 | Proof generated | Proof size | Verifier time | 
| ----------      | --------------- | ---------- | ------------- |
| Winterfell      | 337 ms          | 43.6 KB    | 0.8 ms        |
| Winterfell-wasm | 502 ms          | 43.6 KB    | 1.5 ms        |
| Miden           | 46552 ms        | 86 KB      | 2 ms          |
| Miden-wasm      | 64907 ms        | 86 KB      | 6 ms          |

Computing Fibonacci sequence up to 1048576th term, Proof security: 96 bits (CPU: i5-1135G7)

|                 | Proof generated | Proof size | Verifier time | 
| ----------      | --------------- | ---------- | ------------- |
| Winterfell      | 6398 ms         | 64.4 KB    | 1 ms          |
| winterfell-wasm | 9161 ms         | 64.4 KB    | 2 ms          |

**Computing to 1048576th term using Miden-wasm failed: memory allocation of 33554432 bytes failed**

