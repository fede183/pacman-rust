use bevy::{math::Vec3, prelude::LinearRgba};
use crate::{config::*, sprites::rectagle::{Rectangle, RectangleBundle}};

const WALL_THICKNESS: f32 = 3.;

const LIMIT_HEIGHT: f32 = SQUARE_SIZE * 9.;
const LIMIT_WIGTH: f32 = SQUARE_SIZE * 9.;
const LIMIT_POSITIONS: Vec3 = Vec3::new(0., -SQUARE_SIZE, 2.);

const WALL_LEFT_POSITION: Vec3 = Vec3::new(-LIMIT_WIGTH / 2., LIMIT_POSITIONS.y, LIMIT_POSITIONS.z);
const WALL_RIGHT_POSITION: Vec3 = Vec3::new(LIMIT_WIGTH / 2., LIMIT_POSITIONS.y, LIMIT_POSITIONS.z);
const WALL_UP_POSITION: Vec3 = Vec3::new(LIMIT_POSITIONS.x, SQUARE_SIZE * 3.5, LIMIT_POSITIONS.z);
const WALL_DOWN_POSITION: Vec3 = Vec3::new(LIMIT_POSITIONS.x, -SQUARE_SIZE * 5.5, LIMIT_POSITIONS.z);

const WALL_FILL_COLOR: LinearRgba = LinearRgba::BLACK;
const WALL_BORDER_COLOR: LinearRgba = LinearRgba::WHITE;

pub fn get_wall_bundles() -> Vec<RectangleBundle> {
    
    let mut walls = Vec::new();
    let limit_border_left = Rectangle::new(LIMIT_HEIGHT, WALL_THICKNESS, WALL_LEFT_POSITION, WALL_BORDER_COLOR);
    let limit_border_right = Rectangle::new(LIMIT_HEIGHT, WALL_THICKNESS, WALL_RIGHT_POSITION, WALL_BORDER_COLOR);
    let limit_border_up = Rectangle::new(WALL_THICKNESS, LIMIT_WIGTH, WALL_UP_POSITION, WALL_BORDER_COLOR);
    let limit_border_down = Rectangle::new(WALL_THICKNESS, LIMIT_WIGTH, WALL_DOWN_POSITION, WALL_BORDER_COLOR);

    walls.push(limit_border_left);
    walls.push(limit_border_right);
    walls.push(limit_border_up);
    walls.push(limit_border_down);


    let mut wall_bundles = Vec::new();

    for wall in walls {
        wall_bundles.push(wall.generate_sprite());
    }

    return wall_bundles;
}
