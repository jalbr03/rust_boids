use bevy::prelude::*;
use crate::comp::{BoidWorld, Boid};
use crate::boids::*;
use bevy::math::{vec2, vec3};
use crate::{BOID_COUNT, WINDOW_HEIGHT, WINDOW_WIDTH, COHESION, SEPARATION, ALIGNMENT, WALL_AVOIDANCE, WALL_SIZE};
use bevy::app::Events;
use bevy::input::mouse::MouseButtonInput;
use crate::general::{get_dist_sqr, get_cursor_world_position, is_out_of_bouds};
use bevy::input::ElementState;
use crate::walls::{spawn_wall, Wall, line_collision, get_closest_wall_dist};

pub fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    let mut main_boids = BoidWorld::new("Main");
    main_boids.init(BOID_COUNT);
    let mut copy_boids = BoidWorld::new("Copy");
    copy_boids.init(BOID_COUNT);

    for boid in main_boids.boids.iter() {
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: textures.add(
                    TextureAtlas::from_grid(
                        asset_server.load("boid.png"),
                        Vec2::new(64.0, 32.0), 1, 1
                    )
                ),
                transform: Transform{
                    translation: boid.transform.translation,
                    scale: boid.transform.scale,

                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Boid {
                velocity: boid.velocity,
                direction: boid.direction,
                viewing_dist: boid.viewing_dist,
                wall_checks: boid.wall_checks,
                wall_check_angle: boid.wall_check_angle,
                transform: boid.transform,
            });
    }

    commands.spawn().insert(main_boids);
    commands.spawn().insert(copy_boids);
}

pub fn resize_window(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    window.set_resolution(WINDOW_WIDTH, WINDOW_HEIGHT);

}

pub fn place_wall_system(
    buttons: Res<Input<MouseButton>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<TextureAtlas>>,
    windows: Res<Windows>,
    walls: Query<(&Wall, &Transform, Without<Boid>)>,
) {
    if buttons.pressed(MouseButton::Left) {
        let mouse = get_cursor_world_position(&windows);
        let closest = get_closest_wall_dist(&walls, vec3(mouse.x, mouse.y, 0.0));
        if closest > -WALL_SIZE*2.0 {
            spawn_wall(&mut commands, &mut textures, &asset_server, get_cursor_world_position(&windows), WALL_SIZE);
        }
    }
    if buttons.pressed(MouseButton::Right) {
        // ill do something
    }
}

pub fn update(
    mut main_query: Query<(&mut BoidWorld)>,
    mut boid_query: Query<(&mut Boid, &mut Transform)>,
    wall_query: Query<(&Wall, &Transform, Without<Boid>)>,
    time: Res<Time>,
    // windows: Res<Windows>,
) {
//  getting boids -----------------------------------
    let mut main_boids: Option<Mut<BoidWorld>> = None;
    let mut copy_boids: Option<Mut<BoidWorld>> = None;

    for world in main_query.iter_mut() {
        if world.name == "Main".to_string() {
            main_boids = Option::from(world);
        } else {
            copy_boids = Option::from(world);
        }
    }

    if main_boids.is_none() { return; }
    let mut world = main_boids.unwrap();
    let mut world_copy = copy_boids.unwrap();

    // main loop
    for (current_boid, current_copy) in world.as_ref().boids.iter().zip(&mut world_copy.boids) {
        let mut group_size = 0.0;
        let mut alignment = vec3(0.0, 0.0, 0.0);
        let mut cohesion = vec3(0.0, 0.0, 0.0);
        let mut separation = vec3(0.0, 0.0, 0.0);

        current_copy.transform.rotation = Quat::from_rotation_z(current_copy.direction);
        for next_boid in &world.as_ref().boids {
            if current_boid == next_boid { continue; }
            if distance_to_other(current_boid, next_boid) > current_boid.viewing_dist { continue; }

            group_size += 1.0;

            alignment += next_boid.velocity;

            cohesion += next_boid.transform.translation;

            let sep_dist = get_dist_sqr(current_boid.transform.translation, next_boid.transform.translation);
            separation += (current_boid.transform.translation-next_boid.transform.translation)/sep_dist;
        }

        let mut target_velocity = vec3(0.0, 0.0, 0.0);
        let mut sees_wall = false;

        //check walls
        for i in 0..current_copy.wall_checks as i32 {
            let fi = i as f32;
            let angle_change_amount = current_copy.wall_check_angle/(current_copy.wall_checks-1.0)*2.0;
            let current_angle = if fi%2.0 == 0.0 { -(fi/2.0).ceil() } else { (fi/2.0).floor() };
            let final_angle = angle_change_amount*current_angle + current_copy.direction;

            let is_wall = line_collision(&wall_query, current_copy.transform.translation, final_angle, current_copy.viewing_dist*2.0, 0.01);
            let is_out_of_bounds = is_out_of_bouds(current_copy.transform.translation, final_angle, current_copy.viewing_dist);
            if !is_wall && !is_out_of_bounds {
                if i == 0 { break; }
                target_velocity += vec3(final_angle.cos(), final_angle.sin(), 0.0)*WALL_AVOIDANCE;
                break;
            } else {
                sees_wall = true;
            }
        }

        // finalize
        if group_size > 0.0 && !sees_wall {
            alignment /= group_size;
            cohesion /= group_size;

            target_velocity += (alignment - current_boid.velocity)*ALIGNMENT;
            target_velocity += (cohesion - current_boid.transform.translation)*COHESION;
            target_velocity += separation*SEPARATION;
        }
        if (target_velocity.x == target_velocity.y) && target_velocity.y == 0.0 { target_velocity = current_boid.velocity };
        // end of current boid
        // let cursor = get_cursor_world_position(&windows);

        set_target(target_velocity, current_copy);
        move_to_target(current_copy, &time);
        point_to_velocity(current_copy);

        handle_edges(current_copy);
    }

    // set changes
    world.set_world(&world_copy);
    let mut i = 0;
    for (mut boid, mut transform) in boid_query.iter_mut() {
        boid.transform = world.boids.get(i).unwrap().transform;
        transform.translation = world.boids.get(i).unwrap().transform.translation;
        transform.rotation = world.boids.get(i).unwrap().transform.rotation;
        i += 1;
    }

    // let mut boid = world.boids.get_mut(0).unwrap();
    // boid.set_velocity(&vec2(rand::random::<f32>()-10.0, rand::random::<f32>()-10.0));

    // println!("world: {}, copy: {}", world.boids.get(0).unwrap().direction,  world_copy.boids.get(0).unwrap().direction);
}
