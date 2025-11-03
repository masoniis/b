# 🅱️

## Details

- 🅱️

## How to run the project

Compiling the project requires the **rust toolchain**. If you don't already have rust, installation is platform dependent but very straightforward. See [rust's guide](https://rust-lang.org/tools/install/) for steps. Once the toolchain is ready to go, you should have `cargo` as a command, and running the project is very simple from there:

- Run `cargo run --release` to compile and run in **release mode** (higher fps, optimized)
- Run `cargo run` to compile and run in **debug mode** (lower FPS, logs)

I would recommend release mode unless you are curious to see logging statements as the system runs.

### Usage guide

NOTE: The simulation currently doesn't actually pause, but you can still use `Escape` to unlock and unhide the cursor.

| Key(s)       | Action                       |
| :----------- | :--------------------------- |
| `W`          | Move forward                 |
| `S`          | Move backward                |
| `A`          | Move left                    |
| `D`          | Move right                   |
| `Left Shift` | Move faster                  |
| `Escape`     | Toggle "pause"               |
| `F1` or `1`  | Toggle diagnostics           |
| `F2` or `2`  | Toggle opaque wireframe mode |
| `F3` or `3`  | Toggle chunk borders         |
