#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod data;
mod fine;
mod game;
mod splash;
mod utils;

use bevy::{
    asset::AssetPlugin, core::CorePlugin, core_pipeline::CorePipelinePlugin, input::InputPlugin,
    prelude::*, render::RenderPlugin, sprite::SpritePlugin, text::TextPlugin,
    transform::TransformPlugin, ui::UiPlugin, window::WindowPlugin, winit::WinitPlugin,
};

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Splash,
    Game,
    Fine,
}

#[derive(Default)]
struct GlobalData {
    fine_index: usize,
}

#[derive(Component)]
struct PrepareText;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(WindowDescriptor {
            width: 480.,
            height: 480.,
            vsync: true,
            ..Default::default()
        })
        .init_resource::<GlobalData>()
        .add_plugin(CorePlugin)
        .add_plugin(TransformPlugin)
        .add_plugin(InputPlugin)
        .add_plugin(WindowPlugin::default())
        .add_plugin(AssetPlugin)
        .add_plugin(WinitPlugin)
        .add_plugin(RenderPlugin)
        .add_plugin(CorePipelinePlugin)
        .add_plugin(SpritePlugin)
        .add_plugin(TextPlugin)
        .add_plugin(UiPlugin)
        .add_startup_system(setup)
        .add_state(GameState::Splash)
        .add_plugin(splash::SplashPlugin)
        .add_plugin(game::GamePlugin)
        .add_plugin(fine::FinePlugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());
}

fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in to_despawn.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
