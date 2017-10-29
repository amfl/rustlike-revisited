use specs::{ReadStorage, WriteStorage, System, Join};
use component::{MoveDelta, Position, BaseEntity};

pub struct UpdatePos;

/// System which updates the position of entities.
impl <'a> System<'a> for UpdatePos {
    type SystemData = ( WriteStorage<'a, MoveDelta>,
                        WriteStorage<'a, Position> );

    fn run(&mut self, (mut delta, mut pos): Self::SystemData) {
        for (mut delta, mut pos) in (&mut delta, &mut pos).join() {
            pos.x += delta.dx;
            pos.y += delta.dy;
            delta.dx = 0;
            delta.dy = 0;
        }
    }
}

pub struct RenderSystem;

impl <'a> System<'a> for RenderSystem {
    type SystemData = ( ReadStorage<'a, Position>,
                        ReadStorage<'a, BaseEntity> );

    fn run(&mut self, (pos, ent): Self::SystemData) {
        info!("Rendered a frame.");
    }
}
