use crate::{
    render_world::global_extract::generic_systems::extract_resource::ExtractResource,
    simulation_world::time::world_clock::WorldClockResource,
};
use bevy_ecs::{
    change_detection::DetectChangesMut,
    prelude::Resource,
    system::{Commands, ResMut},
};
use std::f32::consts::PI;

/// A resource that contains the extracted sun data for the render world.
#[derive(Resource, Debug, Default, PartialEq)]
pub struct ExtractedSun {
    pub direction: [f32; 3],
}

/// An extractor that extracts the sun's direction from the simulation world's `WorldClockResource`.
pub struct SunExtractor;

impl ExtractResource for SunExtractor {
    type Source = WorldClockResource;
    type Output = ExtractedSun;

    /// Extracts the sun's direction from the `WorldClockResource` and inserts it into the render world.
    fn extract_and_update(
        commands: &mut Commands,
        source: &Self::Source,
        target: Option<ResMut<Self::Output>>,
    ) {
        let day_night_value = source.day_night_cycle_value();

        // just goes in a circle on the xz plane for now
        let angle = day_night_value * 2.0 * PI;
        let x = angle.cos();
        let z = angle.sin();

        let new_sun = ExtractedSun {
            direction: [x, 0.25, z],
        };

        if let Some(mut target_res) = target {
            target_res.set_if_neq(new_sun);
        } else {
            commands.insert_resource(new_sun);
        }
    }
}
