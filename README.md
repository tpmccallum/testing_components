# testing_components

```bash
compress % cargo component build --target wasm32-wasip2                                                              
  Generating bindings for compress (src/bindings.rs)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.07s
```

```bash
compress % wasmtime component run --env PATH=$PATH --dir . --invoke compress target/wasm32-wasip2/debug/compress.wasm
Error: No such file or directory (os error 2)
```
