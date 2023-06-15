use std::f32::consts::PI;
use amethyst::core::{Time, Transform};
use amethyst::ecs::{Component, DenseVecStorage};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

use crate::display::{ARENA_HEIGHT, ARENA_WIDTH};

pub const TANK_VELOCITY_X: f32 = 35.0;
pub const TANK_VELOCITY_Y: f32 = 35.0;
pub const TANK_WIDTH: f32 = 8.0;
pub const TANK_HEIGHT: f32 = 12.0;


pub enum Color {
    RED,
    GREEN,
}

pub struct Tank {
    pub width: f32,
    pub height: f32,
    pub color: Color,
}

impl Tank {
    pub fn new(color: Color) -> Tank {
        Tank {
            width: TANK_WIDTH,
            height: TANK_HEIGHT,
            color
        }
    }
}

impl Component for Tank {
    type Storage = DenseVecStorage<Tank>;
}

#[derive(SystemDesc)]
pub struct TankSystem;

impl<'s> System<'s> for TankSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Tank>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut transforms, tanks, input, time): Self::SystemData) {
        for (tank, transform) in (&tanks, &mut transforms).join() {
            let (mv_amount, rotation) = match tank.color {
                Color::GREEN => (input.axis_value("green_tank_move").unwrap_or(0.0), input.axis_value("green_tank_rotate").unwrap_or(0.0)),
                Color::RED => (input.axis_value("red_tank_move").unwrap_or(0.0), input.axis_value("red_tank_rotate").unwrap_or(0.0)),
            };
            if mv_amount != 0.0 {
                let mut rotation = transform.rotation().euler_angles().2;
                if rotation < 0.0 {
                    rotation += 2.0 * PI;
                }
                let y_dif = rotation.cos() * mv_amount.clone();
                let x_dif = (rotation.sin() * mv_amount) *-1.0;

                let tank_y = transform.translation().y.clone();
                let tank_x = transform.translation().x.clone();
                transform.set_translation_xyz(
                    (tank_x + ((x_dif * time.delta_seconds()) * TANK_VELOCITY_X)).min(ARENA_WIDTH - (TANK_WIDTH * 0.5)).max(TANK_WIDTH * 0.5),
                    (tank_y + ((y_dif * time.delta_seconds()) * TANK_VELOCITY_Y)).min(ARENA_HEIGHT - (TANK_HEIGHT * 0.5)).max(TANK_HEIGHT * 0.5),
                    0.0
                );
            }
            if rotation != 0.0 {
                let new_rotation = transform.rotation().euler_angles().2 + ((rotation * -1.7) * time.delta_seconds());
                transform.set_rotation_2d(new_rotation);
            }
        }
    }
}