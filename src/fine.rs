use super::data::DATA;
use super::{despawn_screen, GameState, GlobalData};
use bevy::prelude::*;

pub struct FinePlugin;

impl Plugin for FinePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Fine).with_system(fine_setup))
            .add_system_set(SystemSet::on_update(GameState::Fine).with_system(enter_game))
            .add_system_set(
                SystemSet::on_exit(GameState::Fine).with_system(despawn_screen::<OnFineScreen>),
            );
    }
}

#[derive(Component)]
struct OnFineScreen;

fn fine_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    global_data: Res<GlobalData>,
) {
    let font = asset_server.load("fonts/FiraMono-Medium.ttf");

    let fine_text = if global_data.fine_index < 7 {
        format!("Wish you {}.", DATA[global_data.fine_index])
    } else {
        format!("{}.", DATA[global_data.fine_index])
    };

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                margin: Rect {
                    bottom: Val::Auto,
                    left: Val::Px(50.0),
                    right: Val::Px(50.0),
                    top: Val::Auto,
                },
                ..Default::default()
            },
            text: Text::with_section(
                fine_text,
                TextStyle {
                    font: font.clone(),
                    font_size: 24.0,
                    color: Color::WHITE,
                },
                Default::default(),
            ),
            ..Default::default()
        })
        .insert(OnFineScreen);

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                position: Rect {
                    bottom: Val::Px(20.0),
                    left: Val::Px(20.0),
                    ..Default::default()
                },
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            text: Text::with_section(
                "Press Enter to start a new game",
                TextStyle {
                    font: font.clone(),
                    font_size: 16.0,
                    color: Color::WHITE,
                },
                Default::default(),
            ),
            ..Default::default()
        })
        .insert(OnFineScreen);
}

fn enter_game(mut game_state: ResMut<State<GameState>>, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_released(KeyCode::Return) {
        game_state.set(GameState::Game).unwrap();
    }
}
