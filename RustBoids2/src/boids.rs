use bevy::math::*;
use crate::comp::Boid;
use crate::{WINDOW_WIDTH, WINDOW_HEIGHT, BOID_SPEED};
use bevy::prelude::*;
use crate::general::get_dist_sqr;

pub fn move_to_target(boid: &mut Boid, time: &Res<Time>) {
    let mut delta = (time.delta().as_micros() as f32)/10000.0;
    if delta == 0.0 { delta = 1.0; }
    // println!("{}", delta);
    boid.transform.translation += boid.velocity*delta;
}

pub fn set_target(target_vel: Vec3, boid: &mut Boid) {
    let mut normalized = target_vel + boid.velocity;
    normalized = normalized.normalize()*BOID_SPEED;

    boid.velocity += (normalized-boid.velocity)/10.0;
}

pub fn point_to_velocity(mut boid: &mut Boid) {
    let vec1 = vec2(0.0,0.0);
    let vec2 = boid.velocity;
    let x = vec2.x - vec1.x;
    let y = vec2.y - vec1.y;

    boid.direction = y.atan2(x);
}

pub fn handle_edges(boid: &mut Boid) {
    let x = boid.transform.translation.x;
    let y = boid.transform.translation.y;
    if x.abs() > WINDOW_WIDTH/2.0 {
        boid.transform.translation.x = WINDOW_WIDTH/2.0 * -1.0 * x.signum();
    }
    if y.abs() > WINDOW_HEIGHT/2.0 {
        boid.transform.translation.y = WINDOW_HEIGHT/2.0 * -1.0 * y.signum();
    }
}

pub fn distance_to_other(current: &Boid, other: &Boid) -> f32 {
    get_dist_sqr(current.transform.translation, other.transform.translation).sqrt()
}