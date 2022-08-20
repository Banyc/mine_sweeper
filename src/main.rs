use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use board_plugin::{resources::board_options::BoardOptions, BoardPlugin};

fn main() {
    let mut app = App::new();
    app.insert_resource(WindowDescriptor {
        title: "Bevy".to_string(),
        ..Default::default()
    });
    app.add_plugins(DefaultPlugins);
    app.add_plugin(WorldInspectorPlugin::new());
    app.add_startup_system(setup_camera);
    app.add_plugin(BoardPlugin {
        font_path: "fonts/m5x7.ttf".to_string(),
        board_options: BoardOptions::default(),
    });
    app.run();
}

fn setup_camera(mut commands: Commands) {
    // 2D orthographic camera
    commands.spawn_bundle(Camera2dBundle::default());
}
