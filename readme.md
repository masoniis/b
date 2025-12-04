# Voxel engine

## Notes for grading

Easiest way to navigate is the showcase keybinds (number keys 1, 2, and 3!), note that after pressing one it may take a bit for terrain to load in so be patient.

### Important usage warnings/notes

1. UI is EXPENSIVE and may cause lag so be weary of that.
2. Shadows have very low render distance of 32 voxels (didn't have time for cascaded shadow maps) and also have some other small issues.

### Cool things I'm proud of (things to pay attention to)

#### Graphics stuff

1. "Vertex pulling." Each group of 6 vertices that make up a face share a **single 32 bit uint** in a GPU buffer. This required cool GPU stuff and unpacking face data in shaders (packing on CPU of course).
2. Global illumination via the "sun" with directional lighting, and a shadow pass that adds basic hard-shadows using shadow mapping.
3. Approximate **ambient occlusion** based on nearby voxels to a vertex. Provides some depth to the world.
4. Full transparency support via a separate render pass.
5. Custom UI implementation (with `taffy` for computing flexbox layouts and `glyphon` for text heavylifting)
6. Custom fog and sky shaders that define the sky and horizon blending with sun/moon.
7. Convenient texture and voxel definition loading enabling swapping voxel textures easily in the `assets/blocks` folder.
8. Water vertices "wave" up and down if you look at them closer

#### Other stuff

1. Chunk loading uses multi-threaded compute pooling
2. ECS architecture for data-oriented design of the entire system
3. Rendering and simulation run in parallel (only with a max of 1-frame delta though)

### AI usage

1. AI wrote the texture converter in [tools/texture_processor](./tools/texture_processor.rs) (though some adjustments and fixes were made) to add tints to existing pngs and generate a water clear texture.
2. AI was very useful for debugging/fixing some indexing errors regarding ambient occlusion and shader winding order regarding consts seen in the shader code and mesher.
3. AI helped with determinance of some thresholds regarding biomes that are typical of other voxel engines (see [multinoise_biomes.rs](./src/simulation_world/terrain/generators/biome/multinoise_biomes.rs)) though I didn't have enough time to fully incorporate biomes in the terrain painter so we don't really see biomes anyway.
4. AI helped with writing the makefile to (hopefully) work on ubuntu since I don't have access to a ubuntu machine.

### Time spent

Not exactly sure but a lot, likely greater than 100 hours

## How to run the project

On most Linux distros, simply running `make` should work (since a bundled version of rust has been included locally)

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
| `W`           | Move forward                                                                         |
| `S`           | Move backward                                                                        |
| `A`           | Move left                                                                            |
| `D`           | Move right                                                                           |
| `1`           | Jump to scene showcase 1 (water bobbing, shadow showing, sinwave gen)                |
| `2`           | Jump to scene showcase 2 (realistic gen, sunset transition happening)                |
| `3`           | Jump to scene showcase 3 (showcase of 3D gen capabilities, 3D simplex area)          |
| `Left Shift`  | Move faster                                                                          |
| `Mouse Left`  | Break targeted voxel                                                                 |
| `Mouse right` | Place voxel against targeted face                                                    |
| `T`           | Switch terrain generator (only applies to new chunks that generate e.g. from moving) |
| `Left Arrow`  | Jump time backwards (by 30 seconds)                                                  |
| `Right Arrow` | Jump time forwards (by 30 seconds)                                                   |
| `F1` or `u`   | Toggle diagnostics UI (FPS, vert count, coordinates)                                 |
| `F2` or `o`   | Toggle opaque wireframe mode                                                         |
| `F3` or `b`   | Toggle chunk borders                                                                 |
| `Escape`      | Toggle "pause" (locks/unlocks cursor, no _real_ pause currently)                     |

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
