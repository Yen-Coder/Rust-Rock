use specs::prelude::*;

use crate::components::*;

pub struct Physics;

impl<'a> System<'a> for Physics {
    type SystemData = (WriteStorage<'a, Position>, ReadStorage<'a, Velocity>);

    fn run(&mut self, (mut positions, velocities): Self::SystemData) {
        for (position, velocity) in (&mut positions, &velocities).join() {
            if velocity.speed > 0 {
                let (dx, dy) = velocity.direction.to_offset();
                position.0 = position.0.offset(dx * velocity.speed, dy * velocity.speed);
            }
        }
    }
}
