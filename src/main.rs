use bevy::{
    input::common_conditions::input_toggle_active, prelude::*, render::camera::ScalingMode,
};
use bevy_inspector_egui::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use pig::PigPlugin;
use ui::GameUI;

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct Player {
    pub speed: f32,
}

#[derive(Resource)]
pub struct Money(pub f32);

mod pig;
mod ui;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (640.0, 480.0).into(),
                        title: "Logic Farming Roguelike".into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
        )
        .insert_resource(Money(100.0))
        .add_plugins((PigPlugin, GameUI))
        .add_systems(Startup, setup)
        .add_systems(Update, character_movement)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 256.0,
        min_height: 144.0,
    };

    commands.spawn(camera);

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("character.png"),
            ..default()
        },
        Player { speed: 100.0 },
        Name::new("Player"),
    ));
}

fn character_movement(
    mut characters: Query<(&mut Transform, &Player)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, player) in &mut characters {
        let mut dir = Vec2::ZERO;
        dir.x = ((input.pressed(KeyCode::D) as i8) - (input.pressed(KeyCode::A) as i8)) as f32;
        dir.y = ((input.pressed(KeyCode::W) as i8) - (input.pressed(KeyCode::S) as i8)) as f32;

        let z = transform.translation.z;
        transform.translation += Vec3::new(dir.x, dir.y, z) * player.speed * time.delta_seconds();
    }
}
