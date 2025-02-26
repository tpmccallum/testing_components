# testing_components

```bash
cd compress
cargo component build --target wasm32-wasip2
```
                                                            
```
Generating bindings for compress (src/bindings.rs)
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.07s
```

```bash
cd compress
wasmtime run --invoke compress target/wasm32-wasip1/debug/compress.wasm
```


