use criterion::{criterion_group, criterion_main, Criterion};
use glam::IVec3;

use b::simulation_world::{
    biome::load_biome_defs_from_disk,
    chunk::{components::chunk_chord::ChunkCoord, ChunkLod},
    terrain::{
        climate::gentrait::ClimateGenerator,
        generators::climate::climate_noise_gen::ClimateNoiseGenerator, BiomeGenerator,
        BiomeMapComponent, BiomeResultBuilder, DefaultBiomeGenerator,
    },
};

/// Each bench in this benchmark builds off the previous (conceptually speaking).
fn bench_single_chunk_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("Single Chunk Generation");

    // INFO: --------------------------
    //         climate benching
    // --------------------------------

    let climate_noise_generator = ClimateNoiseGenerator::new(12345);
    let origin_chunk_coord = ChunkCoord {
        pos: IVec3::new(0, 0, 0),
    };

    // fixed-seed generator and fixed chunk coord
    group.bench_function("climate_noise_chunk_generation", |b| {
        b.iter(|| {
            climate_noise_generator.generate(origin_chunk_coord.clone());
        })
    });

    // INFO: ------------------------
    //         biome benching
    // ------------------------------

    let biome_generator = DefaultBiomeGenerator::default();
    let biome_registry = load_biome_defs_from_disk();
    let origin_noise = climate_noise_generator.generate(origin_chunk_coord.clone());

    group.bench_function("biome_chunk_generation_full", |b| {
        b.iter(|| {
            // setup
            let biome_map = BiomeMapComponent::new_empty(ChunkLod(0));
            let builder = BiomeResultBuilder::new(biome_map, origin_chunk_coord.clone());

            // chunk gen
            biome_generator.generate_biome_chunk(builder, &origin_noise, &biome_registry);
        })
    });

    group.finish();
}

criterion_group!(benches, bench_single_chunk_generation);
criterion_main!(benches);
