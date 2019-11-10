# closest-pair-wasm
🦀 Rust + 🕸 Wasm = ❤

Find the closest pair of points on a 2-D plane, built with Rust + WebAssembly. The actual algorithm to find the points is inside [closest-pair](closest-pair), see its [README](closest-pair/README.md) for more detail.

## Build and run
Install `wasm-pack`

    cargo install wasm-pack

Then follow these steps

    wasm-pack build
    cd www
    npm install
    npm run start

## Deploy
Go to `./www` and run

    ./node_modules/.bin/webpack

You should get the whole web application in the directory `dist`. Copy the files inside `dist` to your server. Make sure your server is configured so that it serves `.wasm` files with the correct MIME type `application/wasm`.
