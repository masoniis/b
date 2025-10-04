use bevy_ecs::resource::Resource;

/// A trait for a resource that can be extracted from the main world into the render world.
///
/// The `Source` is the resource that exists in the main world.
/// The `Output` is the resource that will be created in the render world.
pub trait ExtractResource {
    type Source: Resource;
    type Output: Resource;

    /// Extracts the resource from the main world and returns the render world version.
    fn extract_resource(source: &Self::Source) -> Self::Output;
}
