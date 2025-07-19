use std::collections::LinkedList;

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

pub fn get_wall_bundles() -> LinkedList<RectangleBundle> {
    
    let mut walls = LinkedList::new();
    let limit_border_left = Rectangle::new(LIMIT_HEIGHT, WALL_THICKNESS, WALL_BORDER_COLOR);
    let limit_border_right = Rectangle::new(LIMIT_HEIGHT, WALL_THICKNESS, WALL_BORDER_COLOR);
    let limit_border_up = Rectangle::new(WALL_THICKNESS, LIMIT_WIGTH, WALL_BORDER_COLOR);
    let limit_border_down = Rectangle::new(WALL_THICKNESS, LIMIT_WIGTH, WALL_BORDER_COLOR);

    walls.push_front((limit_border_left, WALL_LEFT_POSITION));
    walls.push_front((limit_border_right, WALL_RIGHT_POSITION));
    walls.push_front((limit_border_up, WALL_UP_POSITION));
    walls.push_front((limit_border_down, WALL_DOWN_POSITION));


    let mut wall_bundles = LinkedList::new();

    for wall in walls {
        wall_bundles.push_front(wall.0.generate_sprite(wall.1));
    }

    return wall_bundles;
}
