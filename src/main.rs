use bevy::{
    pbr::MeshPipeline, prelude::*, render::{
        mesh::{Indices, PrimitiveTopology},
        render_asset::{RenderAsset, RenderAssetUsages}, render_resource::SpecializedMeshPipeline,
    }
};
mod fly_cam;
mod pause_menu;

fn main() {
    App::new()
        .init_state::<GameState>()
        .add_plugins(DefaultPlugins)
        .add_plugins(pause_menu::menu_plugin)
        .add_plugins(fly_cam::FlyCamPlugin)
        .add_systems(Startup, spawn_grass_tiles)
        .add_systems(Startup, spawn_light)
        .add_systems(Update, toggle_game_state)
        .run();
}

#[derive(States, Clone, Eq, PartialEq, Hash, Debug, Default)]
enum GameState {
    PauseMenu,
    #[default]
    Game,
}

fn toggle_game_state(
    keys: Res<ButtonInput<KeyCode>>,
    curr_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        match curr_state.get() {
            GameState::PauseMenu => next_state.set(GameState::Game),
            GameState::Game => next_state.set(GameState::PauseMenu),
        }
    }
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

fn spawn_light(mut cmd: Commands) {
    cmd.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 10_000_000.0,
            range: 100.0,
            ..default()
        },
        transform: Transform::from_xyz(8., 16., 8.),
        ..default()
    });
}

const GRASS_BLADES: usize = 1024;
const GRASS_BLADE_VERTICES: usize = 15;

fn spawn_grass_tiles(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    cmd.spawn(
        PbrBundle {
            mesh: meshes.add(create_grass_blade_mesh()),
            material: materials.add(Color::GREEN),
            ..default()
        },
    );

    // ground plane
    cmd.spawn(
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(50.0, 50.0)),
            material: materials.add(Color::SILVER),
            ..default()
        },
    );
}

fn create_grass_blade_mesh() -> Mesh {
    let vertices = vec![
        [-0.5, 0.0, 0.0],
        [0.5, 0.0, 0.0],
        [0.4, 0.5, 0.0],
        [-0.4, 0.5, 0.0],
    ];

    let indices = vec![0, 1, 3, 1, 2, 3, 0, 3, 1, 1, 3, 2];
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        vec![
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
        ],
    )
    .with_inserted_indices(Indices::U32(indices))
}

// #[derive(Resource)]
// struct CustomPipeline {
//     shader: Handle<Shader>,
//     mesh_pipeline: MeshPipeline
// }

// impl FromWorld for CustomPipeline {
//     fn from_world(world: &mut World) -> Self {
//         todo!()
//     }
// }

// impl SpecializedMeshPipeline for CustomPipeline {
//     type Key;

//     fn specialize(
//         &self,
//         key: Self::Key,
//         layout: &bevy::render::mesh::MeshVertexBufferLayout,
//     ) -> Result<bevy::render::render_resource::RenderPipelineDescriptor, bevy::render::render_resource::SpecializedMeshPipelineError> {
//         todo!()
//     }
// }

// struct DrawMeshInstanced;

// impl<P: PhaseItem> RenderCommand<P> for DrawMeshInstanced {
// }
