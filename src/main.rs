use bevy::prelude::*;

fn main() {
    App::new()
        .init_state::<GameState>()
        .add_plugins(DefaultPlugins)
        .add_plugins(pause_menu::menu_plugin)
        .add_systems(Startup, setup)
        .add_systems(Update, toggle_game_state)
        .run();
}

#[derive(States, Clone, Eq, PartialEq, Hash, Debug, Default)]
enum GameState {
    PauseMenu,
    #[default]
    Game,
}

fn setup(mut cmd: Commands) {
    cmd.spawn(Camera2dBundle::default());
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

mod pause_menu {
    use super::{despawn_screen, GameState};
    use bevy::{app::AppExit, prelude::*};

    pub fn menu_plugin(app: &mut App) {
        app.add_systems(OnEnter(GameState::PauseMenu), setup)
            .add_systems(Update, button_system)
            .add_systems(
                OnExit(GameState::PauseMenu),
                despawn_screen::<OnPauseMenuScreen>,
            );
    }

    #[derive(Component)]
    struct OnPauseMenuScreen;

    fn setup(mut cmd: Commands, asset_server: Res<AssetServer>) {
        cmd.spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            OnPauseMenuScreen,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn(
                TextBundle::from_section(
                    "House Build Game",
                    TextStyle {
                        font: asset_server.load("fonts/RobotoSlab.ttf"),
                        font_size: 64.,
                        color: Color::WHITE,
                    },
                )
                .with_text_justify(JustifyText::Center)
                .with_background_color(Color::RED.into())
                .with_style(Style {
                    top: Val::Percent(20.),
                    ..Default::default()
                }),
            );
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        top: Val::Percent(30.),
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Quit",
                        TextStyle {
                            font: asset_server.load("fonts/RobotoSlab.ttf"),
                            font_size: 48.,
                            color: Color::BLACK,
                        },
                    ));
                });
        });
    }

    fn button_system(
        mut interaction_query: Query<
            (&Interaction, &mut Style),
            (Changed<Interaction>, With<Button>),
        >,
        mut exit: EventWriter<AppExit>,
    ) {
        for (interaction, mut style) in &mut interaction_query {
            match *interaction {
                Interaction::Pressed => {
                    exit.send(AppExit);
                }
                Interaction::Hovered => {
                    style.width = Val::Px(170.);
                }
                Interaction::None => {
                    style.width = Val::Px(150.);
                }
            }
        }
    }
}
