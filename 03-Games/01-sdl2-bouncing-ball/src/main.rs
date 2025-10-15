use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use std::time::Duration;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const BALL_SIZE: u32 = 20;
const BALL_SPEED: i32 = 5;

struct Ball {
    x: i32,
    y: i32,
    vel_x: i32,
    vel_y: i32,
}

impl Ball {
    fn new() -> Ball {
        Ball {
            x: (WINDOW_WIDTH / 2) as i32,
            y: (WINDOW_HEIGHT / 2) as i32,
            vel_x: BALL_SPEED,
            vel_y: BALL_SPEED,
        }
    }

    fn update(&mut self) {
        // Update position
        self.x += self.vel_x;
        self.y += self.vel_y;

        // Bounce off walls
        if self.x <= 0 || self.x >= (WINDOW_WIDTH - BALL_SIZE) as i32 {
            self.vel_x = -self.vel_x;
        }
        if self.y <= 0 || self.y >= (WINDOW_HEIGHT - BALL_SIZE) as i32 {
            self.vel_y = -self.vel_y;
        }

        // Keep ball within bounds
        self.x = self.x.max(0).min((WINDOW_WIDTH - BALL_SIZE) as i32);
        self.y = self.y.max(0).min((WINDOW_HEIGHT - BALL_SIZE) as i32);
    }

    fn render(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        canvas.set_draw_color(Color::RGB(255, 255, 255)); // White ball
        let rect = Rect::new(self.x, self.y, BALL_SIZE, BALL_SIZE);
        canvas.fill_rect(rect)?;
        Ok(())
    }
}

fn main() -> Result<(), String> {
    // Initialize SDL2
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    // Create window
    let window = video_subsystem
        .window("Bouncing Ball - SDL2 Test", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .expect("Could not initialize video subsystem");

    // Create canvas
    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Could not make a rendering context");

    // Create event pump
    let mut event_pump = sdl_context.event_pump()?;

    // Create ball
    let mut ball = Ball::new();

    // Game loop
    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        // Update game state
        ball.update();

        // Clear screen
        canvas.set_draw_color(Color::RGB(0, 0, 0)); // Black background
        canvas.clear();

        // Render ball
        ball.render(&mut canvas)?;

        // Present frame
        canvas.present();

        // Cap frame rate (roughly 60 FPS)
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}