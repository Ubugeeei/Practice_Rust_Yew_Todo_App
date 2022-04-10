## build
```shell
$ wasm-pack build --target web --out-name wasm --out-dir ./static && miniserve ./static --index index.html
```

## Run server
```shell
$ miniserve ./static --index index.html
```