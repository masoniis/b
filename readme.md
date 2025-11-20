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

### Usage warnings

1. When the loading screen ends, terrain chunks will still be loading, it may take some seconds for everything to appear.
2. My UI implementation is far from perfect in terms of drawing and CPU efficiency, and due to the insightful stats like vertex count, it is EXPENSIVE. I recommend turning it off when you aren't actively paying attention to it.
3. Shadows are (kind of) working but shadow render distance is very low (32 voxels), and also still a bit WIP in general (peter panning problems, etc).
4. There are currently only 2 terrain generators, flat and sinusoidal (can be swapped with `T` as seen in the usage guide). Generation speed of chunks has room for improvement so expect holes in the terrain as you move fast from chunks that are waiting to be loaded in still.

### What remains to be done

- I want to increase the complexity of the terrain generator (noise-based, biomes potentially)
- More interesting textures will definitely be added
- Potentially improve and polish shadow implementation
- Other stuff if I have time:
  - dynamic sky coloring and factoration into global illumination
  - adjustable render distance
  - LODs on chunk meshes (already started, but not sure if it will be finished by the deadline to have any meaningful use)
  - clouds (seems like a rabbit hole),
  - bit-packing memory optimizations for GPU data (normals in 3 bits, etc)
  - etc

## How to run the project

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

## Cool "technical" things

### Graphics stuff

1. Global illumination via the "Sun" with directional lighting
2. Approximate ambient occlusion
3. Multipass rendering pipeline
   - Includes shadows (via well known shadow mapping technique)
   - Includes full transparency support
4. Custom UI implementation (with `taffy` for computing flexbox layouts and `glyphon` for text heavylifting)
5. Custom fog and sky shaders that define the sky and horizon blending.
6. Convenient texture and voxel definition loading enabling swapping voxel textures easily in the `assets/blocks` folder.

### Other stuff

1. Chunk loading uses multi-threaded compute pooling
2. ECS architecture for data-oriented design of the entire system
3. Rendering and simulation run in parallel (only with a max of 1-frame delta though)

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
