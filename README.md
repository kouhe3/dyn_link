# dynlink so you can hot reload

```rust
#[dyn_link(lib)]
extern "C" {
    fn sayhelloworld();
    fn add(a: i32, b: i32) -> i32;
}

```

```rust
fn sayhelloworld() {
    unsafe {
            let lib = libloading::Library::new("lib").unwrap();
            let fun: libloading::Symbol<unsafe extern "C" fn()> =
                lib.get(b"sayhelloworld").unwrap();
            fun();
    }
}
fn add(a: i32, b: i32) -> i32 {
    unsafe {
            let lib = libloading::Library::new("lib").unwrap();
            let fun: libloading::Symbol<unsafe extern "C" fn(a: i32, b: i32) -> i32> =
                lib.get(b"add").unwrap();
            fun(a, b)
    }
}
```
