# 🅱️oxel

[![GitHub Repository](https://img.shields.io/badge/GitHub-Repository-blue?style=flat&logo=github)](https://github.com/masoniis/b)

## Table of contents

- [Cool features](#cool-features)
- [How to run the project](#how-to-run-the-project)
  - [Notes for grading](#notes-for-grading)
  - [Usage guide](#usage-guide)
- [Cool "technical" things](#cool-technical-notes)
  - [Graphics stuff](#graphics-stuff)
  - [Other stuff](#other-stuff)
- [Acknowledgments](#acknowledgments)

## How to run the project

Compiling the project requires the **rust toolchain**. If you don't already have rust, installation is platform dependent but very straightforward. See [rust's guide](https://rust-lang.org/tools/install/) for steps. Once the toolchain is ready to go, you should have `cargo` as a command, and building/running the project is very simple:

- Run `cargo run --release` to compile and run in **release mode** (higher fps, optimized compilation)
- Run `cargo run` to compile and run in **debug mode** (lower FPS, debug tracing, simplified compilation)

I would recommend release mode unless you want to avoid compliation time as FPS is far higher.

### Notes for grading

1. When the loading screen ends, terrain chunks will still be loading, it may take some seconds for everything to appear.
2. My UI implementation is far from perfect in terms of drawing and CPU efficiency, and due to the insightful stats like vertex count, it is EXPENSIVE. I recommend turning it off when you aren't actively paying attention to it.
3. The shadow render distance is very low (32 voxels), and also still a bit WIP in general (peter panning problems, etc).
4. There are currently only 2 terrain generators, flat and sinusoidal (can be swapped with `T` as seen in the usage guide)

### Usage guide

| Key(s)        | Action                                                                               |
| :------------ | :----------------------------------------------------------------------------------- |
| `Escape`      | Toggle "pause" (unlocks cursor)                                                      |
| `W`           | Move forward                                                                         |
| `S`           | Move backward                                                                        |
| `A`           | Move left                                                                            |
| `D`           | Move right                                                                           |
| `Left Shift`  | Move faster                                                                          |
| `Mouse Left`  | Break voxel                                                                          |
| `Mouse right` | Place voxel                                                                          |
| `T`           | Switch terrain generator (only applies to new chunks that generate e.g. from moving) |
| `F1` or `1`   | Toggle diagnostics UI                                                                |
| `F2` or `2`   | Toggle opaque wireframe mode                                                         |
| `F3` or `3`   | Toggle chunk borders                                                                 |

NOTE: The simulation currently doesn't actually pause, but you can still use `Escape` to unlock and unhide the cursor.

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

## Acknowledgments

The "biggest" dependencies this project relies on are...

1. `winit` for an abstraction layer on creating and managing OS windows and events
2. `wgpu` for an abstraction layer on communicating with the gpu and ultimately rendering the graphics
3. `wesl` as a `wgsl` preprocessor, enabling import statements in shaders and other QOL features
4. `glyphon` for handling text rendering (with a glyph atlas), font loading, and vectorization (using underlying `cosmic-text`).
5. `bevy_ecs` for a framework to implement the entity component system the simulation relies on
6. `taffy` for computing UI layouts, particularly flex-box set ups, given a set of input styles.
7. `noise` (rust library) for providing a very simple Simplex noise interface to use in generation.

A full list of dependencies with exact version can be seen in the [cargo.toml](./Cargo.toml).

