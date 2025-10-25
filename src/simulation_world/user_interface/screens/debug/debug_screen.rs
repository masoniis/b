use crate::prelude::*;
use crate::simulation_world::camera::CameraComponent;
use crate::simulation_world::chunk::ChunkChord;
use crate::simulation_world::time::FrameClock;
use crate::simulation_world::user_interface::components::{
    Node, Size, Style, TextAlign, UiBackground, UiText,
};
use crate::simulation_world::user_interface::screens::spawn_root::UiRootNodeResource;
use crate::simulation_world::user_interface::screens::MeshCounterResource;
use bevy_ecs::prelude::*;
use bevy_ecs::relationship::RelatedSpawnerCommands;

// INFO: -------------------------
//         Marker elements
// -------------------------------

/// An enum representing all possible statistic text markers.
pub enum StatMarker {
    // information
    CameraXYZ(CameraChunkChordTextMarker),
    // performance
    Fps(FpsCounterTextElementMarker),
    Memory(MemoryCounterTextElementMarker),
    MeshCount(MeshCountTextMarker),
    VertexCount(VertexCountTextMarker),
    IndexCount(IndexCountTextMarker),
}

/// A marker component for all entities that are part of the diag UI.
#[derive(Component)]
pub struct RootDiagnosticScreenMarker;

/// A marker component for the camera XYZ text element.
#[derive(Component)]
pub struct CameraChunkChordTextMarker;

/// A marker component for the FPS counter text element.
#[derive(Component)]
pub struct FpsCounterTextElementMarker;

/// A marker component for the memory counter text element.
#[derive(Component)]
pub struct MemoryCounterTextElementMarker;

/// A marker component for the total mesh count text element.
#[derive(Component)]
pub struct MeshCountTextMarker;

/// A marker component for the total vertex count text element.
#[derive(Component)]
pub struct VertexCountTextMarker;

/// A marker component for the total triangle count text element.
#[derive(Component)]
pub struct IndexCountTextMarker;

// INFO: -------------------------------------
//         Toggling and creation logic
// -------------------------------------------

/// A run condition that returns true if the diagnostic UI is currently spawned and visible.
pub fn diagnostic_ui_is_visible(query: Query<(), With<RootDiagnosticScreenMarker>>) -> bool {
    !query.is_empty()
}

/// Toggles the debug diagnostics UI by spawning or despawning it.
#[instrument(skip_all)]
pub fn toggle_debug_diagnostics_system(
    // Input
    root_node: Res<UiRootNodeResource>,
    query: Query<Entity, With<RootDiagnosticScreenMarker>>,

    // Needed for init values on spawn
    mesh_stats: Res<MeshCounterResource>,
    camera_query: Query<(&CameraComponent, &ChunkChord)>,
    time_stats: Res<FrameClock>,

    // Output (toggling UI)
    mut commands: Commands,
) {
    if let Ok(ui_entity) = query.single() {
        info!("Despawning Diagnostic UI...");
        commands.entity(ui_entity).despawn();
    } else {
        if let Ok((_, chord)) = camera_query.single() {
            spawn_diagnostic_ui(&mut commands, &root_node, &mesh_stats, &chord, &time_stats);
        } else {
            error!("Cannot spawn Diagnostic UI: No camera with ChunkChord found!");
        }
    }
}

/// Spawns the entire Diagnostic UI and attaches it to the persistent root node.
fn spawn_diagnostic_ui(
    commands: &mut Commands,
    root_node: &Res<UiRootNodeResource>,

    // init stats
    mesh_stats: &Res<MeshCounterResource>,
    camera_chord: &ChunkChord,
    time_stats: &Res<FrameClock>,
) {
    info!("Spawning Diagnostic UI...");
    let root_entity = root_node.0;

    let diagnostic_ui_container = commands
        .spawn((
            RootDiagnosticScreenMarker,
            Node,
            Style {
                position: taffy::style::Position::Absolute,
                width: Size::Percent(100.0),
                height: Size::Percent(100.0),
                flex_direction: taffy::style::FlexDirection::Row,
                justify_content: Some(taffy::JustifyContent::SpaceBetween),
                align_items: Some(taffy::AlignItems::Start),
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            let font_size = 32.0;
            let align = TextAlign::Center;

            // INFO: wrapper for elements on left side of screen
            parent
                .spawn((
                    Node,
                    Style {
                        width: Size::Percent(100.0),
                        height: Size::Percent(100.0),
                        flex_direction: taffy::style::FlexDirection::Column,
                        justify_content: Some(taffy::JustifyContent::Start),
                        align_items: Some(taffy::AlignItems::Start),
                        ..Default::default()
                    },
                ))
                // chunk chord line
                .with_children(|parent| {
                    let chord_line_elements = vec![StatLineElement {
                        prefix: "Camera chunk: ".to_string(),
                        content: camera_chord.to_string(),
                        color: [0.8, 0.8, 0.2, 1.0],
                        marker: StatMarker::CameraXYZ(CameraChunkChordTextMarker),
                    }];
                    spawn_stats_line(parent, chord_line_elements, font_size, align);
                });

            // INFO: wrapper for elements on right side of screen
            parent
                .spawn((
                    Node,
                    Style {
                        width: Size::Percent(100.0),
                        height: Size::Percent(100.0),
                        flex_direction: taffy::style::FlexDirection::Column,
                        justify_content: Some(taffy::JustifyContent::Start),
                        align_items: Some(taffy::AlignItems::End),
                        ..Default::default()
                    },
                ))
                .with_children(|parent| {
                    // fps line
                    let fps_line_elements = vec![StatLineElement {
                        prefix: "FPS: ".to_string(),
                        content: format!("{:.2}", time_stats.smoothed_fps),
                        color: [1.0, 1.0, 1.0, 1.0],
                        marker: StatMarker::Fps(FpsCounterTextElementMarker),
                    }];
                    spawn_stats_line(parent, fps_line_elements, font_size, align);

                    let memory_line_elements = vec![StatLineElement {
                        prefix: "MEM: ".to_string(),
                        content: "0 MB".to_string(),
                        color: [1.0, 1.0, 1.0, 1.0],
                        marker: StatMarker::Memory(MemoryCounterTextElementMarker),
                    }];
                    spawn_stats_line(parent, memory_line_elements, font_size, align);

                    // mesh line
                    let mesh_line_elements = vec![
                        StatLineElement {
                            prefix: "Meshes: ".to_string(),
                            content: mesh_stats.total_meshes.to_string(),
                            color: [0.9, 0.6, 0.6, 1.0],
                            marker: StatMarker::MeshCount(MeshCountTextMarker),
                        },
                        StatLineElement {
                            prefix: " Verts: ".to_string(),
                            content: mesh_stats.total_vertices.to_string(),
                            color: [0.6, 0.8, 0.6, 1.0],
                            marker: StatMarker::VertexCount(VertexCountTextMarker),
                        },
                        StatLineElement {
                            prefix: " Idxs: ".to_string(),
                            content: mesh_stats.total_indices.to_string(),
                            color: [0.6, 0.6, 0.9, 1.0],
                            marker: StatMarker::IndexCount(IndexCountTextMarker),
                        },
                    ];
                    spawn_stats_line(parent, mesh_line_elements, font_size, align);
                });
        })
        .id();

    commands
        .entity(root_entity)
        .add_child(diagnostic_ui_container);
}

/// A data struct to define one part of a multi-part stat line.
pub struct StatLineElement {
    /// A label prefix for the dynamic text (e.g., "FPS: ")
    pub prefix: String,
    /// The initial value for the dynamic text (e.g., "0")
    pub content: String,
    /// The color of the dynamic text
    pub color: [f32; 4],
    /// The marker component, wrapped in our enum.
    pub marker: StatMarker,
}

/// A generic helper to spawn a multi-part statistics line from a Vec of elements.
fn spawn_stats_line(
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    elements: Vec<StatLineElement>,
    font_size: f32,
    text_align: TextAlign,
) {
    parent
        .spawn((
            Node,
            Style {
                padding: 8.0,
                flex_direction: taffy::style::FlexDirection::Row,
                align_items: Some(taffy::style::AlignItems::Center),
                ..Default::default()
            },
            UiBackground::SolidColor {
                color: [0.0, 0.0, 0.0, 0.33],
            },
        ))
        .with_children(|line| {
            let static_color = [0.7, 0.7, 0.7, 1.0];

            for element in elements {
                // static prefix
                if !element.prefix.is_empty() {
                    line.spawn((
                        Node,
                        Style::default(),
                        UiText {
                            content: element.prefix,
                            font_size,
                            color: static_color,
                            align: text_align,
                        },
                    ));
                }

                // dynamic text with marker
                let mut text_entity = line.spawn((
                    Node,
                    Style::default(),
                    UiText {
                        content: element.content,
                        font_size,
                        color: element.color,
                        align: text_align,
                    },
                ));
                match element.marker {
                    StatMarker::CameraXYZ(marker) => text_entity.insert(marker),
                    StatMarker::Fps(marker) => text_entity.insert(marker),
                    StatMarker::Memory(marker) => text_entity.insert(marker),
                    StatMarker::MeshCount(marker) => text_entity.insert(marker),
                    StatMarker::VertexCount(marker) => text_entity.insert(marker),
                    StatMarker::IndexCount(marker) => text_entity.insert(marker),
                };
            }
        });
}
