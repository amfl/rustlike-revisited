extern crate rlr;
extern crate pancurses;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate specs;

use rlr::event::{Event, EventQueue};
use rlr::entity::{Color};
use rlr::map::Map;
use rlr::game_state::GameState;

use rlr::component::{Position, MoveDelta, BaseEntity, Puppeted};
use specs::World;
use specs::DispatcherBuilder;

fn main() {
    // Initialize the logger in the main executable.
    // Libraries will simply include `log` and use the macros.
    // Can always just not set it up and there is very little overhead!
    env_logger::init().unwrap();
    info!("Starting RLR.");

    let win = pancurses::initscr();

    // Specs stuff
    let mut world = World::new();
    world.register::<Position>();
    world.register::<MoveDelta>();
    world.register::<BaseEntity>();
    world.register::<Puppeted>();

    let mut running = true;

    let mut map = Map::new(32, 32);
    let mut mobs = Vec::new();

    let (px, py) = rlr::map_utils::make_map(&mut map, &mut world);

    let player = world.create_entity()
        .with(Puppeted)
        .with(Position { x: px as i32, y: py as i32 })
        .with(MoveDelta { dx: 0, dy: 0 })
        .with(BaseEntity {
                fg: Color::Red,
                bg: Color::Default,
                glyph: '@',
                blocks: true,
                name: String::from("Player"),
            })
        .build();

    let npc = world.create_entity()
        .with(Position { x: 5, y: 5 })
        .with(BaseEntity {
                fg: Color::Blue,
                bg: Color::Default,
                glyph: '$',
                blocks: true,
                name: String::from("Mysterious Glyph"),
            })
        .build();

    let mut entities = vec![player, npc];
    entities.append(&mut mobs);

    let mut renderer = rlr::render_functions::Renderer::new();
    rlr::render_functions::Renderer::static_init();

    let mut game_state = GameState::PlayerTurn;

    // SPECS: Systems + Dispatcher
    let mut update_pos = rlr::system::UpdatePos;
    // let mut render_system = rlr::system::RenderSystem;
    let mut event_system = rlr::system::EventSystem;

    let mut dispatcher = DispatcherBuilder::new()
        .add(event_system, "event_system", &[])
        .add(update_pos, "update_pos", &["event_system"])
        // .add_thread_local(render_system)
        .build();

    world.add_resource(EventQueue(Vec::new()));

    pancurses::noecho();
    pancurses::curs_set(0);

    while running {
        renderer.render_all(&win, &map, &world);

        // Push the input into the world as a resource.
        let input = win.getch();

        if let Some(x) = input {
            if let Some(event) = rlr::input_handlers::handle_keys(x) {
                match event {
                    // Some system events we need to handle here
                    Event::Quit => { running = false; },

                    // But almost everything else can be handled from the ECS
                    // Just chuck it on the event queue ;D
                    other => {
                        let mut x = world.write_resource::<EventQueue>();
                        (*x).0.push(other);
                    },

                    // Event::Movement((dx, dy)) => {
                    //     if let GameState::PlayerTurn = game_state {
                    //         // let pos = {
                    //         //     let player = &entities[0];
                    //         //     (player.x + dx, player.y + dy)
                    //         // };
                    //         // if map.data[pos.1 as usize][pos.0 as usize].walkable {
                    //         //     // if rlr::entity::get_blocking_entities_at(&entities, pos.0, pos.1).len() > 0 {
                    //         //     //     info!("Punt!");
                    //         //     // }
                    //         //     // else {
                    //         //     //     let player = &mut entities[0];
                    //         //     //     player.mov(dx, dy);
                    //         //     // }
                    //         // }
                    //         // game_state = GameState::AITurn;
                    //     }
                    // },

                }
            }
        }

        // if let GameState::AITurn = game_state {
        //     // for ent in entities.iter() {
        //     //     if ent.name != "Player" {
        //     //         info!("The {} ponders the meaning of its existence.", ent.name);
        //     //     }
        //     // }
        //     game_state = GameState::PlayerTurn;
        // }

        dispatcher.dispatch(&world.res);

        // We're all done with the events, so let's clear them out
        {
            let mut x = world.write_resource::<EventQueue>();
            (*x).0 = Vec::new();
        }

        // Maintain dynamically added and removed entities in dispatch.
        // This is what actually executes changes done by `LazyUpdate`.
        world.maintain();

        // renderer.clear_entity(&win, &entities[0]);


    }

    pancurses::endwin();
}
