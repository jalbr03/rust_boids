use bevy::prelude::*;
use crate::comp::Boid;
use crate::general::get_dist_sqr;
use crate::WALL_SIZE;

#[derive(Component)]
pub struct Wall {
    is_wall: bool,
    size: f32,
}

pub fn spawn_wall(commands: &mut Commands, textures: &mut ResMut<Assets<TextureAtlas>>, asset_server: &Res<AssetServer>, pos: Vec2, size_r: f32) {
    commands
        // Spawn a bevy sprite-sheet
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: textures.add(
                TextureAtlas::from_grid(
                    asset_server.load("wall.png"),
                    Vec2::new(256.0, 256.0), 1, 1
                )
            ),
            transform: Transform {
                translation: Vec3::new(pos.x,pos.y, 0.0),
                scale: Vec3::splat((size_r*2.0) / 256.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Wall {
            is_wall: true,
            size: size_r,
        });
}

pub fn line_collision(walls: &Query<(&Wall, &Transform, Without<Boid>)>, mut from_pos: Vec3, cast_direction: f32, line_dist: f32, break_dist: f32) -> bool {
    let mut closest_wall_dist = get_closest_wall_dist(walls, from_pos);
    let mut total_dist = closest_wall_dist;

    while total_dist < line_dist && closest_wall_dist > break_dist {
        total_dist += closest_wall_dist;

        from_pos.x += closest_wall_dist * cast_direction.cos();
        from_pos.y += closest_wall_dist * cast_direction.sin();

        closest_wall_dist = get_closest_wall_dist(walls, from_pos);
    }

    return closest_wall_dist <= break_dist;
}

pub fn get_closest_wall_dist(walls: &Query<(&Wall, &Transform, Without<Boid>)>, from_pos: Vec3) -> f32 {
    let mut closest = 1000000000.0;

    for (_, mut transform, _) in walls.iter() {
        let dist = get_dist_sqr(from_pos, transform.translation).sqrt()-WALL_SIZE*3.0;
        if dist < closest {
            closest = dist;
        }
    }

    return closest;
}
