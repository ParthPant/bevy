mod anchors;
mod color_material;
pub mod entity;
mod margins;
mod node;
mod rect;
mod render;
mod ui_update_system;

pub use anchors::*;
pub use color_material::*;
pub use margins::*;
pub use node::*;
pub use rect::*;
pub use render::*;
pub use ui_update_system::*;

use bevy_app::{stage, AppBuilder, AppPlugin};
use bevy_asset::{AssetStorage, Handle};
use bevy_render::{
    mesh::{shape::Quad, Mesh},
    render_graph::RenderGraph,
    shader::asset_handle_shader_def_system,
};
use glam::Vec2;
use legion::prelude::IntoSystem;

#[derive(Default)]
pub struct UiPlugin;

pub const QUAD_HANDLE: Handle<Mesh> = Handle::from_bytes([
    179, 41, 129, 128, 95, 217, 79, 194, 167, 95, 107, 115, 97, 151, 20, 62,
]);

impl AppPlugin for UiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let mut color_materials = AssetStorage::<ColorMaterial>::new();
        color_materials.add_default(ColorMaterial::default());

        app.add_resource(color_materials)
            .add_system_to_stage(
                stage::POST_UPDATE,
                asset_handle_shader_def_system::<ColorMaterial>.system(),
            )
            .add_system(ui_update_system());

        let resources = app.resources();
        let mut render_graph = resources.get_mut::<RenderGraph>().unwrap();
        render_graph.add_ui_graph(resources);

        let mut meshes = resources.get_mut::<AssetStorage<Mesh>>().unwrap();
        meshes.add_with_handle(
            QUAD_HANDLE,
            Mesh::from(Quad {
                size: Vec2::new(1.0, 1.0),
            }),
        );
    }
}