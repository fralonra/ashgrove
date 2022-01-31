use bevy::prelude::*;

use super::{despawn_screen, GameState};

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Splash).with_system(splash_setup))
            .add_system_set(SystemSet::on_update(GameState::Splash).with_system(enter_game))
            .add_system_set(
                SystemSet::on_exit(GameState::Splash).with_system(despawn_screen::<OnSplashScreen>),
            );
    }
}

#[derive(Component)]
struct OnSplashScreen;

fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let icon = asset_server.load("icons/logo.png");
    let font = asset_server.load("fonts/FiraMono-Medium.ttf");

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                margin: Rect::all(Val::Auto),
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: Color::BLACK.into(),
            ..Default::default()
        })
        .insert(OnSplashScreen)
        .with_children(|parent| {
            parent.spawn_bundle(ImageBundle {
                style: Style {
                    size: Size::new(Val::Px(200.0), Val::Px(200.0)),
                    ..Default::default()
                },
                image: UiImage(icon),
                ..Default::default()
            });
            parent.spawn_bundle(TextBundle {
                style: Style {
                    margin: Rect::all(Val::Px(50.0)),
                    ..Default::default()
                },
                text: Text::with_section(
                    "Press Enter to start the seeking",
                    TextStyle {
                        font: font.clone(),
                        font_size: 20.0,
                        color: Color::WHITE,
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        });
}

fn enter_game(mut game_state: ResMut<State<GameState>>, keys: Res<Input<KeyCode>>) {
    if keys.just_released(KeyCode::Return) {
        game_state.set(GameState::Game).unwrap();
    }
}
