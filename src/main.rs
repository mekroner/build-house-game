use bevy::prelude::*;
mod pause_menu;
mod fly_cam;

fn main() {
    App::new()
        .init_state::<GameState>()
        .add_plugins(DefaultPlugins)
        .add_plugins(pause_menu::menu_plugin)
        .add_plugins(fly_cam::FlyCamPlugin)
        .add_systems(Startup, spawn_grass_tiles)
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

fn spawn_grass_tiles(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    cmd.spawn(
        // ground plane
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(50.0, 50.0)),
            material: materials.add(Color::SILVER),
            ..default()
        },
    );
}
