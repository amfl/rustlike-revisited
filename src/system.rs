extern crate pancurses;

use specs::{ReadStorage, WriteStorage, System, Join, Fetch, FetchMut, Entities};
use component::{MoveDelta, Position, BaseEntity, Puppeted, Blocking};
use event::{Event, InputQueue, MovementIntent};
use map::Map;
use input_handlers;

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

/// Deals with transforming keyboard events into actual game events.
pub struct InputSystem;
impl <'a> System<'a> for InputSystem {
    type SystemData = ( Entities<'a>,
                        ReadStorage<'a, Puppeted>,
                        ReadStorage<'a, Position>,
                        Fetch<'a, InputQueue>,
                        FetchMut<'a, MovementIntent>,
                        );
    fn run(&self, data: Self::SystemData) {
        let (entities, puppeted, position, &input, &movement_intent) = data;
        for keypress in input.0 {
            match input_handlers::handle_keys(keypress) {
                Some(Event::Movement((dx, dy))) => {
                    for (ent, _, _) in (&*entities, puppeted, position).join() {
                        movement_intent.0.push((ent, Event::Movement((dx, dy))));
                    }
                }
            }
        }
    }
}

// pub fn in_location(ent: Entity<'a>, pos: POSITION, (x: i32, y: i32)) -> bool {
//     // TODO Implement
// }

pub struct MovementSystem;
impl <'a> System<'a> for MovementSystem {
    type SystemData = ( Entities<'a>,
                        Fetch<'a, MovementIntent>,
                        WriteStorage<'a, Position>,
                        ReadStorage<'a, Blocking>,
                        ReadStorage<'a, BaseEntity>,
                        );

    fn run(&self, (entities, movement_intent, sPosition, sBlocking, sBaseEnt): Self::SystemData) {
        for movement in movement_intent.0.iter() {
            let mut blocking_ents = Vec::new();

            // Iterate through every blocking entity and store it for later
            for (ent, _, _, _) in (&*entities, &sBlocking, &sPosition, &sBaseEnt).join() {
                blocking_ents.push(ent);
            }

            // For every moving entity...
            for &(ent, (dx, dy)) in movement_intent.iter() {
                let can_move = {
                    // Figure out where the mover wants to move
                    let posa = pos.get(ent).unwrap();

                    info!("This mover lives at {:?}", posa);
                    let (new_x, new_y) = (posa.x + dx, posa.y + dy);

                    // Check that the map isn't blocking it
                    let &tile = map.at(new_x, new_y);
                    if tile.walkable {
                        // Check an entity isn't blocking it
                        let mut blocked_by_ent = false;
                        for &block in blocking_ents.iter() {
                            info!("Checking: {:?} and {:?}", ent, block);
                            if let Some(block_pos) = pos.get(block) {
                                if block_pos.x == new_x && block_pos.y == new_y {
                                    let name = &baseent.get(block).unwrap().name;
                                    info!("Collision! {}", name);
                                    blocked_by_ent = true;
                                    break;
                                }
                            }
                        }
                        !blocked_by_ent
                    }
                    else { false }
                };

                if can_move {
                    // Have to do the mutable borrow here, otherwise we would have a
                    // mutable + immutable borrow at the same time when checking the
                    // blocking entity's position
                    let posa = pos.get_mut(ent).unwrap();
                    posa.x += dx;
                    posa.y += dy;
                }
            }
        }
    }
}

// pub struct EventSystem;

// impl <'a> System<'a> for EventSystem {
//     type SystemData = ( Entities<'a>,
//                         Fetch<'a, EventQueue>,
//                         Fetch<'a, Map>,
//                         ReadStorage<'a, Puppeted>,
//                         WriteStorage<'a, Position>,
//                         ReadStorage<'a, Blocking>,
//                         ReadStorage<'a, BaseEntity>);

//     fn run(&mut self, data: Self::SystemData) {
//         let (entities, events, map, puppet, mut pos, blocking, baseent) = data;
//         for event in events.0.iter() {
//             info!("Detected event: {:?}", event);
//             match event {
//                 },
//                 _ => {}
//             }
//         }
//     }
// }
