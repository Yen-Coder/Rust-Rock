use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{WindowCanvas, Texture};
use specs::prelude::*;

use crate::components::*;

// Type alias for the data needed by the renderer
pub type SystemData<'a> = (
    ReadStorage<'a, Position>,
    ReadStorage<'a, Sprite>,
);

pub fn render(
    canvas: &mut WindowCanvas,
    background: Color,
    textures: &[Texture],
    data: SystemData,
) -> Result<(), String> {
    let (positions, sprites) = data;

    canvas.set_draw_color(background);
    canvas.clear();

    // Get the dimensions of the window
    let (width, height) = canvas.output_size()?;

    // Draw all entities with Position and Sprite components
    for (pos, sprite) in (&positions, &sprites).join() {
        let current_frame = sprite.region;
        
        // Treat the center of the screen as the (0, 0) coordinate
        let screen_position = pos.0 + Point::new(width as i32 / 2, height as i32 / 2);
        
        // Create a rectangle centered on the screen position
        let screen_rect = Rect::from_center(
            screen_position,
            current_frame.width(),
            current_frame.height(),
        );

        canvas.copy(
            &textures[sprite.spritesheet], 
            current_frame, 
            screen_rect
        )?;
    }

    canvas.present();
    
    Ok(())
}