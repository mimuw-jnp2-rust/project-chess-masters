use bevy::prelude::*;

const BOARD_SIZE: u32 = 8;

fn spawn_field(commands: &mut Commands, size: f32, x: f32, y: f32, color: (f32, f32, f32)) {
    let (r, g, b) = color;
    commands.spawn(SpriteBundle {
        transform: Transform::from_xyz(x, y, 1.0),
        sprite: Sprite {
            color: Color::rgb(r, g, b),
            custom_size: Some(Vec2::new(size, size)),
            ..default()
        },
        ..default()
    });
}

// We might use width and height later on during process
// to compute optimal size of field for the screen.
// Optional arguments: (width: u32, height: u32)
pub fn spawn_board(mut commands: Commands, field_size: f32) {
    let start_x = (-1.0) * ((field_size * BOARD_SIZE as f32) / 2.0 - (field_size / 2.0));
    let mut x = start_x;
    let mut y = start_x;
    let mut current_color: (f32, f32, f32);
    for i in 0..BOARD_SIZE {
        for j in 0..BOARD_SIZE {
            if (i + j) % 2 == 0 {
                current_color = (255.0, 255.0, 255.0);
            } else {
                current_color = (0.0, 0.0, 0.0);
            }
            spawn_field(&mut commands, field_size, x, y, current_color);
            x += field_size;
        }
        x = start_x;
        y += field_size;
    }
}
