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
    mut interaction_query: Query<(&Interaction, &mut Style), (Changed<Interaction>, With<Button>)>,
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

