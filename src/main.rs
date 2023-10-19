use bevy::prelude::*;

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
        .add_systems(Startup, setup)
        .add_systems(Update, character_movement)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..default()
        },
        texture: asset_server.load("character.png"),
        ..default()
    });
}

fn character_movement(
    mut characters: Query<(&mut Transform, &Sprite)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let speed = 100.0;

    for (mut transform, _) in &mut characters {
        let mut dir = Vec2::ZERO;
        dir.x = ((input.pressed(KeyCode::D) as i8) - (input.pressed(KeyCode::A) as i8)) as f32;
        dir.y = ((input.pressed(KeyCode::W) as i8) - (input.pressed(KeyCode::S) as i8)) as f32;

        let z = transform.translation.z;
        transform.translation += Vec3::new(dir.x, dir.y, z) * speed * time.delta_seconds();
    }
}
