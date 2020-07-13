# Maccuitar

Little project to practice chord changes and an opportunity to play with web
assembly.

## Usage

Will need to install a few things prior:

```sh
> cargo +nightly install miniserve
> cargo install wasm-pack
```

Then to start:

```sh
> wasm-pack build --target web --out-name wasm --out-dir ./static
> miniserve ./static --index index.html
```
