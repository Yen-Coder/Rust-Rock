use specs::prelude::*;

use crate::components::*;

pub struct Animator;

impl<'a> System<'a> for Animator {
    type SystemData = (
        WriteStorage<'a, MovementAnimation>,
        WriteStorage<'a, Sprite>,
        ReadStorage<'a, Velocity>,
    );

    fn run(&mut self, (mut animations, mut sprites, velocities): Self::SystemData) {
        use self::Direction::*;
        for (animation, sprite, velocity) in (&mut animations, &mut sprites, &velocities).join() {
            if velocity.speed == 0 {
                continue;
            }

            let frames = match velocity.direction {
                Left => &animation.left_frames,
                Right => &animation.right_frames,
                Up => &animation.up_frames,
                Down => &animation.down_frames,
            };

            animation.current_frame = (animation.current_frame + 1) % frames.len();
            *sprite = frames[animation.current_frame].clone();
        }
    }
}
