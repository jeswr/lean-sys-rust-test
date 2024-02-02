# lean-sys test failure repro

Running `cargo build; cargo test` results in

```bash
   Compiling oxrdf4lean v0.1.0 (/home/jesght/Documents/GitHub/oxrdf4lean)
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
   Compiling oxrdf4lean v0.1.0 (/home/jesght/Documents/GitHub/oxrdf4lean)
    Finished test [unoptimized + debuginfo] target(s) in 0.20s
     Running unittests src/lib.rs (target/debug/deps/oxrdf4lean-3ecb2ebbd0f3f266)

running 1 test
error: test failed, to rerun pass `--lib`

Caused by:
  process didn't exit successfully: `/home/jesght/Documents/GitHub/oxrdf4lean/target/debug/deps/oxrdf4lean-3ecb2ebbd0f3f266` (signal: 11, SIGSEGV: invalid memory reference)
```

The same error occurs if we use other functions like `lean_mk_string` in `lib/src.rs` as well.
