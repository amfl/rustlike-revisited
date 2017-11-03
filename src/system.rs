extern crate pancurses;

use specs::{ReadStorage, WriteStorage, System, Join, Fetch, Entities};
use component::{MoveDelta, Position, BaseEntity, Puppeted, Blocking};
use event::{Event, EventQueue};
use map::Map;

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

pub struct EventSystem;

impl <'a> System<'a> for EventSystem {
    type SystemData = ( Entities<'a>,
                        Fetch<'a, EventQueue>,
                        Fetch<'a, Map>,
                        ReadStorage<'a, Puppeted>,
                        WriteStorage<'a, Position>,
                        ReadStorage<'a, Blocking> );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, events, map, puppet, mut pos, blocking) = data;
        for event in events.0.iter() {
            info!("Detected event: {:?}", event);
            match event {
                // If a movement has occured...
                &Event::Movement((dx, dy)) => {
                    let mut ents = Vec::new();

                    // Find every puppeted entity and store it in a vec
                    for (puppet, mut posa, ent) in (&puppet, &mut pos, &*entities).join() {
                        let &tile = map.at(posa.x + dx, posa.y + dy);
                        if tile.walkable {
                            ents.push((ent, (posa.x + dx, posa.y + dy)));
                            // Iterate over all entities and see if there's one blocking your way.
                            // type MoarData = ( ReadStorage<'a, Blocking>,
                            //                   ReadStorage<'a, Position> );

                            // for (blk, mut posb) in (&blocking, &mut pos).join() {
                            //     // if let Some(Blocking)
                            //     info!("Blocking Entity at: {:?}", posb);
                            // }

                            posa.x += dx;
                            posa.y += dy;
                        }
                    }
                    for (entity, (new_x, new_y)) in ents {
                        let moveto = pos.get(entity);

                        // Iterate through every blocking entity
                        for (mut posa, _, ent) in (&pos, &blocking, &*entities).join() {
                            if posa.x == new_x && posa.y == new_y && ent != entity {
                                info!("Collision!");
                            }
                        }
                    }
                },
                _ => {}
            }
        }
    }
}
