# badapple-wasm

Play Bad Apple through DOM manipulation.

Horrible performance, mostly because of relying on js to sleep until next frame although I'm sure it can be optimized in many other ways.

[Live version](https://xoko14.github.io/badapple-wasm/)

## Build and run

Needs `wasm-pack`.

```shell
wasm-pack build --target web
```

Serve `index.html`, `style.css`, `data/` and `pkg/`.
