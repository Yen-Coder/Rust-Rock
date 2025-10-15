use specs::prelude::*;
use crate::components::*;
use super::MovementCommand;

const PLAYER_MOVEMENT_SPEED: i32 = 20;

pub struct Keyboard;

impl<'a> System<'a> for Keyboard {
    type SystemData = (
        ReadExpect<'a, Option<MovementCommand>>,
        ReadStorage<'a, KeyboardControlled>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, (movement_command, keyboard_controlled, mut velocities): Self::SystemData) {
        // Get the movement command if there is one
        let movement_command = match &*movement_command {
            Some(movement_command) => movement_command,
            None => return, // no change
        };

        for (_, velocity) in (&keyboard_controlled, &mut velocities).join() {
            match movement_command {
                &MovementCommand::Move(direction) => {
                    velocity.speed = PLAYER_MOVEMENT_SPEED;
                    velocity.direction = direction;
                },
                MovementCommand::Stop => velocity.speed = 0,
            }
        }
    }
}