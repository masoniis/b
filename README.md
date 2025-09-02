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

- Run `make` (compiles and runs)

<details>
<summary>How can this be a Rust and C project at the same time?</summary>
This project is written entirely in Rust, but it's configured to compile into a C-compatible static library. This allows the core Rust logic to be called from a simple C main function, enabling you to build and run the entire project using standard C tools like make and gcc without needing to install the Rust toolchain.
</details>

---

Option 3: requires **GCC**

- Just copy the compile command from the `Makefile ` and run that

---
