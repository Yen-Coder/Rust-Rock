use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{WindowCanvas, Texture};
use sdl2::rect::{Point, Rect};
use sdl2::image::{self, LoadTexture, InitFlag};
use std::time::Duration;
use std::collections::VecDeque;

const PLAYER_MOVEMENT_SPEED: i32 = 20;
const FRAME_RATE: u32 = 20; // More standard frame rate is 60 FPS
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    /// Returns the movement delta for this direction
    const fn to_offset(self) -> (i32, i32) {
        match self {
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
        }
    }

    /// Check if this direction is horizontal
    const fn is_horizontal(self) -> bool {
        matches!(self, Direction::Left | Direction::Right)
    }

    /// Check if this direction is vertical  
    const fn is_vertical(self) -> bool {
        matches!(self, Direction::Up | Direction::Down)
    }
}

#[derive(Debug)]
struct Player {
    position: Point,
    sprite: Rect,
    speed: i32,
    direction: Direction,
    current_frame: u8,
}

/// Returns the row of the spritesheet corresponding to the given direction
fn direction_spritesheet_row(direction: Direction) -> i32 {
    use self::Direction::*;
    match direction {
        Up => 3,
        Down => 0,
        Left => 1,
        Right => 2,
    }
}


impl Player {
    fn new(start_position: Point) -> Self {
        Self {
            position: start_position,
            sprite: Rect::new(0, 0, 26, 36),
            speed: 0,
            direction: Direction::Right,
            current_frame: 0,
        }
    }

    fn update(&mut self) {
        if self.speed > 0 {
            let (dx, dy) = self.direction.to_offset();
            self.position = self.position.offset(dx * self.speed, dy * self.speed);
        }
    }

    fn set_movement(&mut self, direction: Option<Direction>) {
        match direction {
            Some(dir) => {
                self.direction = dir;
                self.speed = PLAYER_MOVEMENT_SPEED;
            }
            None => {
                self.speed = 0;
            }
        }
    }
}

/// Handles robust input state tracking with opposite key cancellation
#[derive(Debug)]
struct InputHandler {
    direction_stack: VecDeque<Direction>, // Handles the order of key presses (complex)
    horizontal_balance: i32, // +1 for right, -1 for left, 0 for balanced/none
    vertical_balance: i32,   // +1 for down, -1 for up, 0 for balanced/none
}

impl InputHandler {
    fn new() -> Self {
        Self {
            direction_stack: VecDeque::new(),
            horizontal_balance: 0,
            vertical_balance: 0,
        }
    }

    fn press_key(&mut self, direction: Direction) {
        // Remove direction if already present (prevents duplicates)
        self.direction_stack.retain(|&d| d != direction);
        
        // Add to front (most recent)
        self.direction_stack.push_front(direction);
        
        // Update balance counters
        self.update_balance(direction, 1);
    }

    fn release_key(&mut self, direction: Direction) {
        // Remove from stack
        self.direction_stack.retain(|&d| d != direction);
        
        // Update balance counters
        self.update_balance(direction, -1);
    }

    fn update_balance(&mut self, direction: Direction, delta: i32) {
        match direction {
            Direction::Left => self.horizontal_balance -= delta,
            Direction::Right => self.horizontal_balance += delta,
            Direction::Up => self.vertical_balance -= delta,
            Direction::Down => self.vertical_balance += delta,
        }
    }

    /// Returns the current effective direction based on input state
    fn get_effective_direction(&self) -> Option<Direction> {
        // Find the most recent direction that isn't cancelled by opposite keys
        self.direction_stack
            .iter()
            .find(|&&direction| self.is_direction_active(direction))
            .copied()
    }

    fn is_direction_active(&self, direction: Direction) -> bool {
        if direction.is_horizontal() {
            self.horizontal_balance != 0
        } else if direction.is_vertical() {
            self.vertical_balance != 0
        } else {
            false // This should never happen with our current Direction enum
        }
    }
}

/// Game state management
struct Game {
    player: Player,
    input_handler: InputHandler,
}

impl Game {
    fn new() -> Self {
        Self {
            player: Player::new(Point::new(0, 0)),
            input_handler: InputHandler::new(),
        }
    }

    fn handle_key_event(&mut self, keycode: Keycode, is_pressed: bool) {
        let direction = match keycode {
            Keycode::Left => Direction::Left,
            Keycode::Right => Direction::Right,
            Keycode::Up => Direction::Up,
            Keycode::Down => Direction::Down,
            _ => return, // Ignore other keys
        };

        if is_pressed {
            self.input_handler.press_key(direction);
        } else {
            self.input_handler.release_key(direction);
        }
    }

    fn update(&mut self) {
        let effective_direction = self.input_handler.get_effective_direction();
        self.player.set_movement(effective_direction);
        self.player.update();
    }
}

fn render(
    canvas: &mut WindowCanvas,
    color: Color,
    texture: &Texture,
    player: &Player,
) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.clear();

    let (width, height) = canvas.output_size()?;
    let (frame_width, frame_height) = player.sprite.size();
    let current_frame = Rect::new(
        player.sprite.x() + frame_width as i32 * i32::from(player.current_frame),
        player.sprite.y() + frame_height as i32 * direction_spritesheet_row(player.direction),
        frame_width,
        frame_height,
    );

     // Treat the center of the screen as the (0, 0) coordinate
    let screen_position = player.position + Point::new(width as i32 / 2, height as i32 / 2);
    let screen_rect = Rect::from_center(screen_position, frame_width, frame_height);
    canvas.copy(texture, current_frame, screen_rect)?;

    canvas.present();
  

    Ok(())
}

fn main() -> Result<(), String> {
    // Initialize SDL2
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;

    // Create window and canvas
    let window = video_subsystem
        .window("Game Tutorial", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| format!("Could not initialize video subsystem: {}", e))?;

    let mut canvas = window
        .into_canvas()
        .build()
        .map_err(|e| format!("Could not create canvas: {}", e))?;

    // Load texture
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("assets/bardo.png")?;

    // Initialize game state
    let mut game = Game::new();
    let mut event_pump = sdl_context.event_pump()?;
    let mut frame_counter = 0u8;

    // Game loop
    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } 
                | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
                Event::KeyDown { keycode: Some(keycode), repeat: false, .. } => {
                    game.handle_key_event(keycode, true);
                }
                Event::KeyUp { keycode: Some(keycode), repeat: false, .. } => {
                    game.handle_key_event(keycode, false);
                }
                _ => {}
            }
        }

        // Update game state
        game.update();

        // Render
        frame_counter = frame_counter.wrapping_add(1);
        let background_color = Color::RGB(frame_counter, 64, 255 - frame_counter);
        render(&mut canvas, background_color, &texture, &game.player)?;

        // Frame rate control
        std::thread::sleep(Duration::from_nanos(1_000_000_000 / u64::from(FRAME_RATE)));
    }

    Ok(())
}
