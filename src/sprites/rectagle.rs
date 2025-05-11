use bevy::prelude::*;

pub struct Rectangle {
    height: f32, 
    wigth: f32, 
    fill_color: LinearRgba, 
}

#[derive(Bundle)]
pub struct RectangleBundle {
    sprite: Sprite,
    transform: Transform
}

impl Rectangle {
    pub fn new(height: f32, wigth: f32, fill_color: LinearRgba) -> Rectangle {
        Rectangle {
            height,
            wigth,
            fill_color,
        }
    }

    pub fn generate_sprite(&self, positions: Vec3) -> RectangleBundle {
        let sprite = Sprite::from_color(self.fill_color, Vec2 {
                    x: self.wigth,
                    y: self.height,
                });
        RectangleBundle {
            sprite,
            transform: Transform::from_xyz(positions.x, positions.y, positions.z),
        }
    }
}

pub struct RectangleWithBorder {
    fill: Rectangle,
    border: Rectangle,
}

impl RectangleWithBorder {
    pub fn new(height: f32, wigth: f32, border_size: f32, fill_color: LinearRgba, border_color: LinearRgba) -> RectangleWithBorder {
        RectangleWithBorder {
            fill: Rectangle::new(height, wigth, fill_color),
            border: Rectangle::new(height + border_size, wigth + border_size, border_color),
        }
    }

    pub fn spawn(&self, commands: &mut Commands, positions: Vec3) {
        let border_sprite = self.border.generate_sprite(positions);
        let fill_sprite = self.fill.generate_sprite(Vec3 { x: positions.x, y: positions.y, z: positions.z + 1. });
        
        commands.spawn(border_sprite);
        commands.spawn(fill_sprite);
    }
}
