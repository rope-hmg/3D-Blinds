# Tropical Blinds

This is a demo of the tropical blinds project. Built by watching their video.

# Development

In one shell run:

```bash
cd ./app
cargo watch -s "wasm-pack build . --target web"
```

In another shell run:

```bash
cd ./web
npm i
npx vite
```

On the first run, the web page will fail to load because the wasm won't have compiled yet. It should automatically start, but if not try refreshing the page. It can take a minute or so (Rust is slow).