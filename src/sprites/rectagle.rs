use bevy::prelude::*;

pub struct Rectangle {
    pub height: f32,
    pub wigth: f32,
    pub position: Vec3,
    pub fill_color: LinearRgba,
}

#[derive(Bundle)]
pub struct RectangleBundle {
    pub sprite: Sprite,
    transform: Transform
}

impl Rectangle {
    pub fn new(height: f32, wigth: f32, position: Vec3, fill_color: LinearRgba) -> Rectangle {
        Rectangle {
            height,
            wigth,
            position,
            fill_color,
        }
    }

    pub fn generate_sprite(&self) -> RectangleBundle {
        let sprite = Sprite::from_color(self.fill_color, Vec2 {
                    x: self.wigth,
                    y: self.height,
                });
        RectangleBundle {
            sprite,
            transform: Transform::from_xyz(self.position.x, self.position.y, self.position.z),
        }
    }
}

pub struct RectangleWithBorder {
    fill: Rectangle,
    border: Rectangle,
}

impl RectangleWithBorder {
    pub fn new(height: f32, wigth: f32, border_size: f32, position: Vec3, fill_color: LinearRgba, border_color: LinearRgba) -> RectangleWithBorder {
        RectangleWithBorder {
            fill: Rectangle::new(height, wigth, Vec3 { x: position.x, y: position.y, z: position.z + 1. }, fill_color),
            border: Rectangle::new(height + border_size, wigth + border_size, position, border_color),
        }
    }

    pub fn spawn(&self, commands: &mut Commands) {
        let border_sprite = self.border.generate_sprite();
        let fill_sprite = self.fill.generate_sprite();
        
        commands.spawn(border_sprite);
        commands.spawn(fill_sprite);
    }
}
