# dynlink so you can hot reload

```rust
#[dyn_link(lib)]
extern "C" {
    fn sayhelloworld();
    fn add(a: i32, b: i32) -> i32;
}

```

WARN: just a sample of idea, maybe panic

you maybe want use https://lib.rs/crates/subsecond https://lib.rs/crates/hot-lib-reloader
