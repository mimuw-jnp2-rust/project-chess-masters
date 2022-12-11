use bevy::prelude::Vec2;

#[derive(Debug, Copy, Clone)]
pub struct Bounds2 {
    pub position: Vec2,
    pub size: Vec2,
}

impl Bounds2 {
    pub fn in_bounds(&self, coords: Vec2) -> bool {
        coords.x >= self.position.x
            && coords.y >= self.position.y
            && coords.x <= self.position.x + self.size.x
            && coords.y <= self.position.y + self.size.y
    }
}

fn spawn_field(
    parent: &mut ChildBuilder,
    size: f32,
    x: f32,
    y: f32,
    coords: Coordinates,
    color: (f32, f32, f32),
) {
    let (r, g, b) = color;

    parent
        .spawn(SpriteBundle {
            transform: Transform::from_xyz(x, y, 1.0),
            sprite: Sprite {
                color: Color::rgb(r, g, b),
                custom_size: Some(Vec2::new(size, size)),
                ..default()
            },
            ..default()
        })
        .insert(Name::new(format!("Field ({}, {})", coords.x, coords.y)))
        .insert(coords);
}

// We might use width and height later on during process
// to compute optimal size of field for the screen.
// Optional arguments: (width: u32, height: u32)
pub fn spawn_fields(parent: &mut ChildBuilder, size: f32) {
    let start_x = (-1.0) * ((FIELD_SIZE * BOARD_SIZE as f32) / 2.0 - (FIELD_SIZE / 2.0));
    let mut x = start_x;
    let mut y = start_x;
    let mut current_color: (f32, f32, f32);
    for i in 0..BOARD_SIZE {
        for j in 0..BOARD_SIZE {
            let coordinates = Coordinates {
                x: i as i8,
                y: j as i8,
            };
            if (i + j) % 2 == 0 {
                current_color = (255.0, 255.0, 255.0);
            } else {
                current_color = (0.0, 0.0, 0.0);
            }
            spawn_field(&mut parent, FIELD_SIZE, x, y, coordinates, current_color);
            x += FIELD_SIZE;
        }
        x = start_x;
        y += FIELD_SIZE;
    }
}
