# fn coerce
```fn()``` coercion

### Example

```Rust
unsafe extern "system" fn function_item() {}
/// FnCo::<Parameter count, Is unsafe, ABI wrapped by lit_t macro>
let _fn_poiter = FnCo::<0, true, lit_t!("system")>::co(function_item);
```

This may be helpful when dealing with those crate which has abstraction over fn pointers. For example, *[closure-ffi](https://crates.io/crates/closure-ffi)*.