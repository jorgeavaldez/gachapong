use bevy::prelude::*;

#[derive(Component)]
enum Direction {
    Idle,
    Up,
    Down,
    Left,
    Right,
}

pub struct ChickenPlugin;

impl Plugin for ChickenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_chicken)
            .add_systems(Update, (bind_keys, move_chicken));
    }
}

fn setup_chicken(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let img_path = "Characters/Free Chicken Sprites.png".to_string();
    let texture_handle = asset_server.load(&img_path);
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(16.0, 16.0),
        2,
        4,
        Some(Vec2::new(5., 5.)),
        None,
    );

    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn((
        SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: 0,
                ..default()
            },
            transform: Transform::from_xyz(0., 0., 1.).with_scale(Vec3::splat(2.0)),
            texture_atlas: texture_atlas_handle.clone(),
            ..default()
        },
        Direction::Idle,
    ));
}

fn bind_keys(keyboard_input: Res<Input<KeyCode>>, mut pos: Query<&mut Direction>) {
    for mut dir in &mut pos {
        if keyboard_input.pressed(KeyCode::W) {
            *dir = Direction::Up;
        }

        if keyboard_input.pressed(KeyCode::A) {
            *dir = Direction::Left;
        }

        if keyboard_input.pressed(KeyCode::S) {
            *dir = Direction::Down;
        }

        if keyboard_input.pressed(KeyCode::D) {
            *dir = Direction::Right;
        }

        if keyboard_input.get_just_released().len() > 0 {
            *dir = Direction::Idle;
        }
    }
}

const VELOCITY: f32 = 300.;

fn move_chicken(time: Res<Time>, mut pos: Query<(&mut Direction, &mut Transform)>) {
    for (dir, mut transform) in &mut pos {
        match *dir {
            Direction::Up => transform.translation.y += VELOCITY * time.delta_seconds(),
            Direction::Down => transform.translation.y -= VELOCITY * time.delta_seconds(),
            Direction::Left => {
                transform.translation.x -= VELOCITY * time.delta_seconds();
                transform.rotation = Quat::from_rotation_y(std::f32::consts::PI);
            }
            Direction::Right => {
                transform.translation.x += VELOCITY * time.delta_seconds();
                transform.rotation = Quat::default();
            }
            Direction::Idle => (),
        }
    }
}
