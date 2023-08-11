use bevy::prelude::*;

use rand::seq::SliceRandom;

pub fn make_tiles(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let img_path = "Tilesets/Grass.png".to_string();
    let texture_handle = asset_server.load(&img_path);
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(16.0, 16.0),
        10,
        10,
        Some(Vec2::new(0., -1.)),
        None,
    );

    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let leftmost_border = 1280 / 2 * -1;
    let topmost_border = 720 / 2 + 16;

    let rightmost_border = 1280 / 2 + 16;
    let bottommost_border = 720 / 2 * -1;

    let grass_tiles: Vec<usize> = (0..5).chain(11..15).collect();
    let mut rng = rand::thread_rng();

    let mut sprites = vec![];
    for y in (bottommost_border..topmost_border).step_by(16) {
        for x in (leftmost_border..rightmost_border).step_by(16) {
            let tile_index = grass_tiles.choose(&mut rng).unwrap();
            sprites.push(SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index: *tile_index,
                    ..default()
                },
                transform: Transform::from_xyz(x as f32, y as f32, 0.),
                // .with_scale(Vec3::splat(2.0)),
                texture_atlas: texture_atlas_handle.clone(),
                ..default()
            });
        }
    }

    commands.spawn_batch(sprites);
}

