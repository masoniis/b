# 🅱️oxel

[![GitHub Repository](https://img.shields.io/badge/GitHub-Repository-blue?style=flat&logo=github)](https://github.com/masoniis/b)
![Rust Version](https://img.shields.io/badge/rustc-1.88.0%2B-orange.svg)

## Table of contents

- [Notes for grading](#notes-for-grading)
  - [Usage warnings](#usage-warnings)
  - [What remains to be done](#what-remains-to-be-done)
- [How to run the project](#how-to-run-the-project)
  - [Usage guide](#usage-guide)
- [Cool "technical" things](#cool-technical-things)
  - [Graphics stuff](#graphics-stuff)
  - [Other stuff](#other-stuff)
- [Acknowledgments](#acknowledgments)

## Notes for grading

### Important usage warnings/notes

1. MOUSE CAN BE UNLOCKED WITH `ESCAPE`

2. UI is EXPENSIVE. I recommend turning it off when you aren't actively paying attention to it.
3. Shadows have very low render distance of 32 voxels (didn't have time for cascaded shadow maps) and also have some other small issues.
4. Chunk generation speed is not extremely fast (swap generator type with `T`). It is easy to overwhelm it by moving fast depending on hardware.

### Cool things I'm proud of (things to pay attention to)

#### Graphics stuff

1. "Vertex pulling." Each group of 6 vertices that make up a face share a single 32 bit float in a GPU buffer.
2. Global illumination via the "Sun" with directional lighting, and a shadow pass that adds (somewhat scuffed) shadows
3. Approximate ambient occlusion based on nearby voxels to a vertex.
4. Full transparency support via a separate render pass.
5. Custom UI implementation (with `taffy` for computing flexbox layouts and `glyphon` for text heavylifting)
6. Custom fog and sky shaders that define the sky and horizon blending with sun/moon.
7. Convenient texture and voxel definition loading enabling swapping voxel textures easily in the `assets/blocks` folder.
8. Water vertices "wave" up and down if you look at them closer

#### Other stuff

1. Chunk loading uses multi-threaded compute pooling
2. ECS architecture for data-oriented design of the entire system
3. Rendering and simulation run in parallel (only with a max of 1-frame delta though)

## How to run the project

1. On linux, simply running `make` should work (since a bundled version of rust has been included locally)

Otherwise...

Compiling the project requires **Rust 1.88 or newer**.

> [!TIP]
> If you don't have Rust, check out [the official guide](https://rust-lang.org/tools/install/) for installing it.
>
> - Assuming `rustup` is being used (eg, from the official guide above), the project's `rust-toolchain.toml` will automatically handle versioning to match 1.88.
> - Any other installation method will have to manually ensure the version 1.88 or newer.

To run, `cargo` can be used like any standard Rust project:

- Run `cargo run --release` to compile and run in **release mode** (higher fps, optimized compilation)
- Run `cargo run` to compile and run in **debug mode** (lower FPS, debug tracing, simplified compilation)

### Usage guide

| Key(s)        | Action                                                                               |
| :------------ | :----------------------------------------------------------------------------------- |
| `Escape`      | Toggle "pause" (locks/unlocks cursor, no _real_ pause currently)                     |
| `W`           | Move forward                                                                         |
| `S`           | Move backward                                                                        |
| `A`           | Move left                                                                            |
| `D`           | Move right                                                                           |
| `Left Shift`  | Move faster                                                                          |
| `Mouse Left`  | Break voxel                                                                          |
| `Mouse right` | Place voxel                                                                          |
| `T`           | Switch terrain generator (only applies to new chunks that generate e.g. from moving) |
| `F1` or `1`   | Toggle diagnostics UI (FPS, vert count, coordinates)                                 |
| `F2` or `2`   | Toggle opaque wireframe mode                                                         |
| `F3` or `3`   | Toggle chunk borders                                                                 |

## Acknowledgments

The "biggest" dependencies this project relies on are...

1. `winit` for an abstraction layer on creating and managing OS windows and events
2. `wgpu` for an abstraction layer on communicating with the gpu and ultimately rendering the graphics
3. `wesl` as a `wgsl` preprocessor, enabling import statements in shaders and other QOL features
4. `glyphon` for handling text rendering (with a glyph atlas), font loading, and vectorization (using underlying `cosmic-text`)
5. `bevy_ecs` for a framework to implement the entity component system the simulation relies on
6. `taffy` for computing UI layouts, particularly flex-box set ups, given a set of input styles.
7. `noise` (rust library) for providing a very simple Simplex noise interface to use in generation.

A full list of dependencies with exact version can be seen in the [cargo.toml](./Cargo.toml).
