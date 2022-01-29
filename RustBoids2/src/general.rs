use bevy::prelude::*;
use bevy::math::{vec2};
use crate::{WINDOW_WIDTH, WINDOW_HEIGHT};

pub fn get_dist_sqr(pos1: Vec3, pos2: Vec3) -> f32{
    let a = pos1.x-pos2.x;
    let b = pos1.y-pos2.y;
    return a*a+b*b;
}

pub fn is_out_of_bouds(pos: Vec3, angle: f32, dist: f32) -> bool {
    let new_pos = vec2(pos.x + angle.cos()*dist, pos.y + angle.sin()*dist);

    new_pos.x.abs() > WINDOW_WIDTH/2.0 || new_pos.y.abs() > WINDOW_HEIGHT/2.0
}

pub fn get_cursor_world_position(windows: &Res<Windows>) -> Vec2 {
    let mut cursor_position = if let Some(cursor_position) = windows
        .get_primary()
        .and_then(|window| window.cursor_position())
    {
        cursor_position
    } else {
        return vec2(0.0, 0.0);
    };
    let window = windows.get_primary().unwrap();
    cursor_position -= vec2(window.width()/2.0, window.height()/2.0);
    return cursor_position;
}

pub fn point_direction2d(vec1: Vec2, vec2: Vec2) -> f32 {
    let x = vec2.x - vec1.x;
    let y = vec2.y - vec1.y;
    return y.atan2(x);
}