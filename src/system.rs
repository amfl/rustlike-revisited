extern crate pancurses;

use specs::{ReadStorage, WriteStorage, System, Join, Fetch};
use component::{MoveDelta, Position, BaseEntity, Puppeted};
use event::{Event, IOEvent};

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

// pub struct RenderSystem {
//     pub win: pancurses::Window,
// }

// impl RenderSystem {
//     pub fn getch(self: &Self) -> Option<pancurses::Input> {
//         self.win.getch()
//     }
//     pub fn refresh(self: &Self) -> i32 {
//         self.win.refresh()
//     }
// }

// impl <'a> System<'a> for RenderSystem {
//     type SystemData = ( ReadStorage<'a, Position>,
//                         ReadStorage<'a, BaseEntity> );

//     fn run(&mut self, (pos, ent): Self::SystemData) {
//         info!("Rendered a frame.");

//         self.win.refresh();
//     }
// }

pub struct InputSystem;

impl <'a> System<'a> for InputSystem {
    type SystemData = ( Fetch<'a, IOEvent>,
                        ReadStorage<'a, Puppeted>,
                        WriteStorage<'a, MoveDelta> );

    fn run(&mut self, (input, puppet, mut mov): Self::SystemData) {
        info!("Moving player");
    }
}
