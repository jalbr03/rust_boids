use bevy::prelude::*;
use bevy::math::{vec3};
use crate::{WINDOW_WIDTH, WINDOW_HEIGHT};
use std::f32::consts::PI;

#[derive(PartialEq)]
#[derive(Copy)]
#[derive(Clone)]
#[derive(Component)]
pub struct Boid {
    pub velocity: Vec3,
    pub direction: f32,
    pub wall_checks: f32,
    pub wall_check_angle: f32,
    pub viewing_dist: f32,
    pub transform: Transform,
}

#[derive(Component)]
pub struct BoidWorld {
    pub name: String,
    pub boids: Vec<Boid>,
}

impl BoidWorld {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            boids: Vec::new(),
        }
    }

    pub fn set_world(&mut self, other_world: &BoidWorld) {
        self.boids.clone_from_slice(&other_world.boids);
    }

    pub fn init(&mut self, number_of_boids: i32) {
        for _ in 0..number_of_boids {
            self.boids.push(Boid{
                velocity: vec3(rand::random::<f32>()*2.0-1.0, rand::random::<f32>()*2.0-1.0, 0.0),
                direction: 0.0,
                wall_checks: 13.0,
                wall_check_angle: PI*0.95,
                viewing_dist: 100.0,
                transform: Transform {
                    translation: Vec3::new(rand::random::<f32>()*WINDOW_WIDTH-WINDOW_WIDTH*0.5, rand::random::<f32>()*WINDOW_HEIGHT-WINDOW_HEIGHT*0.5, 0.0),
                    scale: Vec3::splat(0.3),
                    // rotation: Quat::from_rotation_z(PI/2.0),
                    ..Default::default()
                },
            })
        }
    }
}