use super::data::DATA;
use super::utils::{is_inputable_char, to_lowercase};
use super::{despawn_screen, GameState, GlobalData};
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use rand::Rng;

const CHAR_ENTRY: char = '?';

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Game>()
            .add_system_set(SystemSet::on_enter(GameState::Game).with_system(game_setup))
            .add_system_set(
                SystemSet::on_update(GameState::Game)
                    .with_system(handle_input.label("handle_input"))
                    .with_system(handle_choice.label("handle_choice").after("handle_input"))
                    .with_system(display),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Game)
                    .with_system(game_end)
                    .with_system(despawn_screen::<OnGameScreen>),
            );
    }
}

#[derive(Component)]
enum ButtonAction {
    A,
    B,
}

#[derive(Component)]
struct OnGameScreen;

#[derive(Component)]
struct TextChoiceA;

#[derive(Component)]
struct TextChoiceB;

#[derive(Component)]
struct TextInfo;

#[derive(Component)]
struct TextInput;

#[derive(Debug, Default)]
struct Game {
    active_choice_index: usize,
    active_data_index: usize,
    active_data_global_index: usize,
    active_data_len: usize,
    choices: [char; 2],
    chosen_char: char,
    datas: [Vec<char>; 2],
    info_type: InfoType,
    init_data0_index: usize,
    init_data1_index: usize,
    input_text: String,
    is_first_choice: bool,
    seeking_index: usize,
}

#[derive(Debug, PartialEq)]
enum InfoType {
    None,
    Fine,
    Neverending,
    Tedious,
}

impl Default for InfoType {
    fn default() -> Self {
        InfoType::None
    }
}

fn game_setup(mut commands: Commands, asset_server: Res<AssetServer>, mut game: ResMut<Game>) {
    let font = asset_server.load("fonts/FiraMono-Medium.ttf");

    let mut rng = rand::thread_rng();
    let data_count = DATA.len();
    let rand0 = rng.gen_range(0..data_count);
    game.datas[0] = String::from(DATA[rand0]).chars().collect();
    loop {
        let rand1 = rng.gen_range(0..data_count);
        if rand1 != rand0 {
            game.datas[1] = String::from(DATA[rand1]).chars().collect();
            game.init_data0_index = rand0;
            game.init_data1_index = rand1;
            break;
        }
    }

    game.choices[0] = to_lowercase(game.datas[0][0]);
    game.choices[1] = to_lowercase(game.datas[1][0]);

    game.active_choice_index = 0;
    game.active_data_index = 0;
    game.active_data_global_index = 0;
    game.active_data_len = 0;
    game.chosen_char = CHAR_ENTRY;
    game.info_type = InfoType::None;
    game.input_text.clear();
    game.is_first_choice = true;
    game.seeking_index = 0;

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
        .insert(OnGameScreen)
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        margin: Rect::all(Val::Px(50.0)),
                        ..Default::default()
                    },
                    text: Text::with_section(
                        "",
                        TextStyle {
                            font: font.clone(),
                            font_size: 20.0,
                            color: Color::WHITE,
                        },
                        Default::default(),
                    ),
                    ..Default::default()
                })
                .insert(TextInfo);
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        margin: Rect::all(Val::Auto),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    color: Color::BLACK.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(ButtonBundle {
                            style: Style {
                                margin: Rect::all(Val::Px(50.0)),
                                ..Default::default()
                            },
                            color: Color::BLACK.into(),
                            ..Default::default()
                        })
                        .insert(ButtonAction::A)
                        .with_children(|parent| {
                            parent
                                .spawn_bundle(TextBundle {
                                    text: Text::with_section(
                                        game.choices[0],
                                        TextStyle {
                                            font: font.clone(),
                                            font_size: 80.0,
                                            color: Color::YELLOW,
                                        },
                                        Default::default(),
                                    ),
                                    ..Default::default()
                                })
                                .insert(TextChoiceA);
                        });
                    parent
                        .spawn_bundle(ButtonBundle {
                            style: Style {
                                margin: Rect::all(Val::Px(50.0)),
                                ..Default::default()
                            },
                            color: Color::BLACK.into(),
                            ..Default::default()
                        })
                        .insert(ButtonAction::B)
                        .with_children(|parent| {
                            parent
                                .spawn_bundle(TextBundle {
                                    text: Text::with_section(
                                        game.choices[1],
                                        TextStyle {
                                            font: font.clone(),
                                            font_size: 80.0,
                                            color: Color::YELLOW,
                                        },
                                        Default::default(),
                                    ),
                                    ..Default::default()
                                })
                                .insert(TextChoiceB);
                        });
                });
            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        margin: Rect::all(Val::Px(50.0)),
                        ..Default::default()
                    },
                    text: Text::with_section(
                        game.input_text.clone(),
                        TextStyle {
                            font: font.clone(),
                            font_size: 20.0,
                            color: Color::WHITE,
                        },
                        Default::default(),
                    ),
                    ..Default::default()
                })
                .insert(TextInput);
        });
}

fn display(
    game: Res<Game>,
    mut query: QuerySet<(
        QueryState<&mut Text, With<TextChoiceA>>,
        QueryState<&mut Text, With<TextChoiceB>>,
        QueryState<&mut Text, With<TextInfo>>,
        QueryState<&mut Text, With<TextInput>>,
    )>,
) {
    for mut text in query.q0().iter_mut() {
        text.sections[0].value = format!("{}", game.choices[0]);
    }
    for mut text in query.q1().iter_mut() {
        text.sections[0].value = format!("{}", game.choices[1]);
    }

    for mut text in query.q2().iter_mut() {
        text.sections[0].value = format!(
            "{}",
            match game.info_type {
                InfoType::Fine => "Great. You have revealed the truth.\nPress any key to continue.",
                InfoType::Tedious =>
                    "Wait...\nDo you think you've pressed\ntoo many letters?\nTry press Esc...",
                _ => "Press one of the following letter\nthen you might find the truth.",
            }
        );
    }

    for mut text in query.q3().iter_mut() {
        text.sections[0].value = format!("{}", game.input_text);
    }
}

fn handle_choice(mut game: ResMut<Game>) {
    if game.info_type == InfoType::Fine {
        return;
    }

    let c = game.chosen_char;
    if c != CHAR_ENTRY {
        if c == game.choices[0] || c == game.choices[1] {
            game.input_text.push(c);

            if game.is_first_choice {
                game.active_choice_index = if c == game.choices[0] { 0 } else { 1 };
                game.active_data_index = game.active_choice_index;
                game.active_data_global_index = if game.active_choice_index == 0 {
                    game.init_data0_index
                } else {
                    game.init_data1_index
                };
                game.active_data_len = game.datas[game.active_data_index].len();
                game.is_first_choice = false;
            }

            if c != game.choices[game.active_choice_index] {
                game.info_type = InfoType::Neverending;
            }
            if game.seeking_index == game.active_data_len - 1 {
                if game.info_type != InfoType::Neverending {
                    game.info_type = InfoType::Fine;
                }
            }
            if game.info_type == InfoType::Neverending {
                if game.seeking_index >= 10 {
                    game.info_type = InfoType::Tedious;
                }
            }

            if game.info_type == InfoType::Fine {
                return;
            }

            game.seeking_index += 1;

            let mut rng = rand::thread_rng();
            let rand_letter = rng.gen_range(b'a'..=b'z') as char;
            let rand0 = rng.gen_range(0..=1);
            let rand1 = 1 - rand0;
            game.choices[rand0] = rand_letter;
            if game.info_type != InfoType::None {
                game.choices[rand1] = rng.gen_range(b'a'..=b'z') as char;
            } else {
                let mut c = game.datas[game.active_data_index][game.seeking_index];
                loop {
                    if !is_inputable_char(c) {
                        game.input_text.push(c);
                        game.seeking_index += 1;
                        c = game.datas[game.active_data_index][game.seeking_index];
                    } else {
                        break;
                    }
                }
                game.choices[rand1] = to_lowercase(c);
                game.active_choice_index = rand1;
            }
        }
    }

    game.chosen_char = CHAR_ENTRY;
}

fn handle_input(
    mut game: ResMut<Game>,
    mut game_state: ResMut<State<GameState>>,
    keys: Res<Input<KeyCode>>,
    mut key_events: EventReader<KeyboardInput>,
    interaction_query: Query<(&Interaction, &ButtonAction), (Changed<Interaction>, With<Button>)>,
    mut query: QuerySet<(
        QueryState<&Text, With<TextChoiceA>>,
        QueryState<&Text, With<TextChoiceB>>,
    )>,
) {
    if game.info_type == InfoType::Fine {
        use bevy::input::ElementState;
        for e in key_events.iter() {
            match e.state {
                ElementState::Released => {
                    game_state.set(GameState::Fine).unwrap();
                    return;
                }
                _ => {}
            }
        }
    }

    if keys.just_released(KeyCode::Escape) {
        game_state.set(GameState::Splash).unwrap();
        return;
    }

    let keys = keys.get_just_released();
    for key in keys.into_iter() {
        match key {
            KeyCode::A => game.chosen_char = 'a',
            KeyCode::B => game.chosen_char = 'b',
            KeyCode::C => game.chosen_char = 'c',
            KeyCode::D => game.chosen_char = 'd',
            KeyCode::E => game.chosen_char = 'e',
            KeyCode::F => game.chosen_char = 'f',
            KeyCode::G => game.chosen_char = 'g',
            KeyCode::H => game.chosen_char = 'h',
            KeyCode::I => game.chosen_char = 'i',
            KeyCode::J => game.chosen_char = 'j',
            KeyCode::K => game.chosen_char = 'k',
            KeyCode::L => game.chosen_char = 'l',
            KeyCode::M => game.chosen_char = 'm',
            KeyCode::N => game.chosen_char = 'n',
            KeyCode::O => game.chosen_char = 'o',
            KeyCode::P => game.chosen_char = 'p',
            KeyCode::Q => game.chosen_char = 'q',
            KeyCode::R => game.chosen_char = 'r',
            KeyCode::S => game.chosen_char = 's',
            KeyCode::T => game.chosen_char = 't',
            KeyCode::U => game.chosen_char = 'u',
            KeyCode::V => game.chosen_char = 'v',
            KeyCode::W => game.chosen_char = 'w',
            KeyCode::X => game.chosen_char = 'x',
            KeyCode::Y => game.chosen_char = 'y',
            KeyCode::Z => game.chosen_char = 'z',
            KeyCode::Key0 => game.chosen_char = '0',
            KeyCode::Key1 => game.chosen_char = '1',
            KeyCode::Key2 => game.chosen_char = '2',
            KeyCode::Key3 => game.chosen_char = '3',
            KeyCode::Key4 => game.chosen_char = '4',
            KeyCode::Key5 => game.chosen_char = '5',
            KeyCode::Key6 => game.chosen_char = '6',
            KeyCode::Key7 => game.chosen_char = '7',
            KeyCode::Key8 => game.chosen_char = '8',
            KeyCode::Key9 => game.chosen_char = '9',
            _ => {}
        }
    }

    for (interaction, button_action) in interaction_query.iter() {
        match *interaction {
            Interaction::Clicked => match button_action {
                ButtonAction::A => {
                    for text in query.q0().iter() {
                        game.chosen_char = text.sections[0].value.chars().collect::<Vec<_>>()[0];
                    }
                }
                ButtonAction::B => {
                    for text in query.q1().iter() {
                        game.chosen_char = text.sections[0].value.chars().collect::<Vec<_>>()[0];
                    }
                }
            },
            _ => {}
        }
    }
}

fn game_end(mut global_data: ResMut<GlobalData>, game: Res<Game>) {
    if game.info_type == InfoType::Fine {
        global_data.fine_index = game.active_data_global_index;
    }
}
