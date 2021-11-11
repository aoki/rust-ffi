
```bash
clang -c -o target/debug/native/fib.o c_src/fib.c
ar r target/debug/deps/libfib.a target/debug/native/fib.o
```

```bash
RUSTFLAGS='-L target/debug/deps' cargo run
```
