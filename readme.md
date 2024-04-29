# rust-hyper-toggle

Testing rust and hyper (HTTP lib) by building a minimal api proxy over Toggle.io API.

## Run with watch

> Requires `cargo-watch`, install via `cargo install cargo-watch`

```sh
cargo watch -c -w src -x run
```

### Crates to test

- http client [reqwest](https://docs.rs/reqwest/latest/reqwest/)
- web framework (built on hyper.rs) [warp](https://docs.rs/warp/latest/warp/)

### Resources

- test render.com for hosting <https://dashboard.render.com/>
- rustlang book <https://doc.rust-lang.org/book/>
- hyper server guide <https://hyper.rs/guides/1/server/echo/>
- jsr.io source code <https://github.com/jsr-io/jsr/blob/main/api/src/main.rs>
- hyper examples repo <https://github.com/hyperium/hyper/tree/master/examples>
