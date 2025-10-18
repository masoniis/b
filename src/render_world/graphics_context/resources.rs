use bevy_ecs::prelude::Resource;
use derive_more::{Deref, DerefMut};
use std::sync::Arc;

#[derive(Resource, Clone, Deref, DerefMut)]
pub struct RenderDevice(pub Arc<wgpu::Device>);

#[derive(Resource, Clone, Deref, DerefMut)]
pub struct RenderQueue(pub Arc<wgpu::Queue>);

#[derive(Resource, Clone, Deref, DerefMut)]
pub struct RenderAdapter(pub Arc<wgpu::Adapter>);

#[derive(Resource, Clone, Deref, DerefMut)]
pub struct RenderSurface(pub Arc<wgpu::Surface<'static>>);

#[derive(Resource, Clone, Deref, DerefMut)]
pub struct RenderSurfaceConfig(pub wgpu::SurfaceConfiguration);
