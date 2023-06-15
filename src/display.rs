use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};
use crate::entities::tanks::*;

pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

pub struct Display;

impl SimpleState for Display {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let sprite_sheet_handle = load_sprite_sheet(world);

        initialise_camera(world);
        initialise_tanks(world, sprite_sheet_handle);
    }
}

// Load the sprite sheet necessary to render
// The texture is the pixel data
// texture_handle is a cloneable reference to the texture
fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {

    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "sprites/tank_trouble_sprites.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "sprites/tank_trouble_sprites.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

// Setup camera to cover screen and (0, 0) in the bottom left
fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build()
    ;
}


// Initialize the tank entities
fn initialise_tanks(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    // Green tank sprite is first and Red tank is second
    let green_sprite_render = SpriteRender::new(sprite_sheet_handle.clone(), 0);
    let red_sprite_render = SpriteRender::new(sprite_sheet_handle, 1);

    let mut green_transform = Transform::default();
    let mut red_transform = Transform::default();

    green_transform.set_translation_xyz((TANK_WIDTH * 0.5) + 2.0, (ARENA_HEIGHT - (TANK_HEIGHT * 0.5)) - 2.0, 0.0);
    red_transform.set_translation_xyz((ARENA_WIDTH - (TANK_WIDTH * 0.5)) - 2.0, (TANK_HEIGHT * 0.5) + 2.0, 0.0);

    // Create Green Tank
    world
        .create_entity()
        .with(green_sprite_render)
        .with(Tank::new(Color::GREEN))
        .with(green_transform)
        .build()
    ;

    world
        .create_entity()
        .with(red_sprite_render)
        .with(Tank::new(Color::RED))
        .with(red_transform)
        .build()
    ;
}