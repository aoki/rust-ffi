# C-API

## Build(mac)

```sh
clang main.c -L target/debug/ -l c_api
export DYLD_LIBRARY_PATH=./target/debug/:$DYLD_LIBRARY_PATH
./a.out
```
