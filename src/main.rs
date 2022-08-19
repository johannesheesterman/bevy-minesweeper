use bevy::prelude::*;
mod boardplugin;

#[cfg(feature = "debug")]
use bevy_inspector_egui::WorldInspectorPlugin;
use boardplugin::BoardPlugin;

fn main() {
    let mut app = App::new();

    app.insert_resource(WindowDescriptor {
        title: "Mine Sweeper!".to_string(),
        width: 700.,
        height: 800.,
        ..Default::default()
    });
    app.add_plugins(DefaultPlugins);
    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin::new());
    app.add_plugin(BoardPlugin);
    app.add_startup_system(camera_setup);

    app.run();
}


fn camera_setup(mut commands: Commands) {
    // 2D orthographic camera
    commands.spawn_bundle(Camera2dBundle::default());
}