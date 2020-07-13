#!/bin/bash
set -ex

wasm-pack build --target web --out-name wasm --out-dir ./static
miniserve ./static --index index.html &
firefox http://127.0.0.1:8080

wait

