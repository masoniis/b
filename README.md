# 🅱️

## Details

- 🅱️
- 🅱️
- 🅱️

## How to run the project

Option 1: requires the **rust toolchain**

- Run `cargo run` (compiles and runs)

---

Option 2: requires **GCC** and **make**

> NOTE: This project is written entirely in Rust, but it's configured to compile into a C-compatible static library, and those static libraries have been placed into the `cridge` folder. This method of running the project is mostly a workaround for convenience, and the preferred way to run the code is with `cargo run`, but this should (probably) work too.

- Run `make` (compiles and runs)
- This outputs the binary at `cbridge/main`, so you can manually re-run it with `./cbridge/main`
