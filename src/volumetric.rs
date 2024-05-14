use bevy::{
    prelude::*,
    math::Vec3,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
};

pub struct VolumetricPlugin;

impl Plugin for VolumetricPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, spawn_volume)
        .add_plugins(MaterialPlugin::<VolumetricMaterial>::default())
        ;
    }
}

#[derive(Component,Debug)]
pub struct VolumetricImageParams{
    pub cols: u32,
    pub rows: u32,
    pub scale: Vec3,
}

fn spawn_volume(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<VolumetricMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // cube
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Cuboid::default()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        material: materials.add(VolumetricMaterial {
            color: Color::WHITE,
            color_texture: Some(asset_server.load("405b7308-6495-42e6-8ee9-1e2aefa43d83.png")),
            alpha_mode: AlphaMode::Blend,
        }),
        ..default()
    });
}

//Custom material
// This struct defines the data that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct VolumetricMaterial {
    #[uniform(0)]
    color: Color,
    #[texture(1)]
    #[sampler(2)]
    color_texture: Option<Handle<Image>>,
    alpha_mode: AlphaMode,
}

/// The Material trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material api docs for details!
impl Material for VolumetricMaterial {
    fn fragment_shader() -> ShaderRef {
        "volumetric.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}
