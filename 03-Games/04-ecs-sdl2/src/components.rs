use specs::prelude::*;
use specs_derive::Component;
use sdl2::rect::{Point, Rect};
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    /// Returns the movement delta for this direction
    pub const fn to_offset(self) -> (i32, i32) {
        match self {
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
        }
    }

    /// Check if this direction is horizontal
    pub const fn is_horizontal(self) -> bool {
        matches!(self, Direction::Left | Direction::Right)
    }

    /// Check if this direction is vertical  
    pub const fn is_vertical(self) -> bool {
        matches!(self, Direction::Up | Direction::Down)
    }
}

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct KeyboardControlled;

/// The current position of a given entity
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position(pub Point);

/// The current speed and direction of a given entity
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Velocity {
    pub speed: i32,
    pub direction: Direction,
}

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Sprite {
    /// The specific spritesheet to render from
    pub spritesheet: usize,
    /// The current region of the spritesheet to be rendered
    pub region: Rect,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct MovementAnimation {
    // The current frame in the animation of the direction this entity is moving in
    pub current_frame: usize,
    pub up_frames: Vec<Sprite>,
    pub down_frames: Vec<Sprite>,
    pub left_frames: Vec<Sprite>,
    pub right_frames: Vec<Sprite>,
}

// Player marker component to identify the player entity
#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct Player;

// Input handling
#[derive(Debug)]
pub struct InputHandler {
    direction_stack: VecDeque<Direction>,
    horizontal_balance: i32,
    vertical_balance: i32,
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            direction_stack: VecDeque::new(),
            horizontal_balance: 0,
            vertical_balance: 0,
        }
    }

    pub fn press_key(&mut self, direction: Direction) {
        self.direction_stack.retain(|&d| d != direction);
        self.direction_stack.push_front(direction);
        self.update_balance(direction, 1);
    }

    pub fn release_key(&mut self, direction: Direction) {
        self.direction_stack.retain(|&d| d != direction);
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

    pub fn get_effective_direction(&self) -> Option<Direction> {
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
            false
        }
    }
}
