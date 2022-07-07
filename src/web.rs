use std::borrow::BorrowMut;
use std::sync::{Arc, Mutex};

use gotham::handler::FileOptions;
use gotham::helpers::http::response::{create_response, create_temporary_redirect};
use gotham::hyper::{Body, Response, StatusCode};
use gotham::middleware::state::StateMiddleware;
use gotham::pipeline::{single_middleware, single_pipeline};
use gotham::prelude::*;
use gotham::router::builder::{build_router, DrawRoutes};
use gotham::router::builder::DefineSingleRoute;
use gotham::router::Router;
use gotham::state::{FromState, State};
use serde::Serialize;
use serde_derive::*;

use crate::loader;
use crate::world::World;

pub fn run(worldfile: &str, app_path: &str, addr: &str) {
    println!("Setting up world");
    let mut world = loader::load_world(worldfile);
    world.add_initial_plants_and_creatures();
    println!("Listening for requests at http://{}", addr);
    let _ = gotham::start(addr.to_string(), router(app_path, WorldWrapper::new(world)));
}

fn router(app_path: &str, world: WorldWrapper) -> Router {
    let middleware = StateMiddleware::new(world);
    let (chain, pipelines) = single_pipeline(single_middleware(middleware));
    build_router(chain, pipelines, |route| {
        route.get("/")
            .to(|state| { let r = create_temporary_redirect(&state, "/ui/worldmap.html"); (state, r) });
        route.get("/favicon.png")
            .to_file(&format!("{}/favicon.png", app_path));
        route.get("/ui/*")
            .to_dir(FileOptions::new(app_path)
                .with_cache_control("no-cache")
                .with_gzip(true)
                .build()
            );
        route.get("/data/worldmap")
            .with_query_string_extractor::<WorldmapQueryStringExtractor>()
            .to(get_worldmap);
    })
}


fn get_worldmap(mut state: State) -> (State, Response<Body>) {
    let cycles = WorldmapQueryStringExtractor::cycles(&mut state);
    let wrapper = WorldWrapper::borrow_from(&state);
    wrapper.do_cycles(cycles);
    let worldmap = wrapper.get_worldmap();
    let response = create_json_response(&state, StatusCode::OK, &worldmap).unwrap();
    (state, response)
}

fn create_json_response<S: Serialize>(state: &State, status: StatusCode, data: &S)
                                      -> Result<Response<Body>, serde_json::Error> {
    serde_json::to_string(data).map(|json_str| {
        create_response(state, status, mime::APPLICATION_JSON, json_str.into_bytes())
    })
}




#[derive(Clone, StateData)]
struct WorldWrapper {
    mutex: Arc<Mutex<World>>,
}

impl WorldWrapper {
    pub fn new(world: World) -> Self {
        Self {
            mutex: Arc::new(Mutex::new(world)),
        }
    }

    pub fn do_cycles(&self, n: u64) {
        let mut guard = self.mutex.lock().unwrap();
        let world = guard.borrow_mut();

        world.do_cycles(n);
    }


    pub fn get_worldmap(&self) -> WorldmapDoc {
        let mut guard = self.mutex.lock().unwrap();
        let world = guard.borrow_mut();

        let creatures = world.terrain.all_creatures_with_pos().iter().map(|((x, y), creature)| {
            CreatureDoc {
                x: *x, y: *y,
                b: creature.bearing, ep: creature.ep, pc: creature.pc,
                program: creature.pp_program(),
                adult: creature.age() > (world.params.creature_start_ep + world.params.eat_ep) as u64,
            }
        }).collect();
        let plants = world.terrain.all_plants_with_pos().iter().map(|((x, y), plant)| {
            PlantDoc {
                x: *x,
                y: *y,
                ep: plant.ep
            }
        }).collect();

        WorldmapDoc { world_size: world.params.world_size, cycle: world.cycle, creatures, plants }
    }

}


#[derive(Deserialize, StateData, StaticResponseExtender)]
struct WorldmapQueryStringExtractor {
    c: Option<u64>,
}

impl WorldmapQueryStringExtractor {
    fn cycles(state: &mut State) -> u64 {
        let query_param = WorldmapQueryStringExtractor::take_from(state);
        query_param.c.unwrap_or(10)
    }
}

#[derive(Serialize, Clone)]
struct WorldmapDoc {
    #[serde(rename = "worldSize")]
    world_size: u32,
    cycle: u64,
    plants: Vec<PlantDoc>,
    creatures: Vec<CreatureDoc>
}

#[derive(Serialize, Clone)]
struct PlantDoc {
    x: u32,
    y: u32,
    ep: u32
}

#[derive(Serialize, Clone)]
struct CreatureDoc {
    x: u32,
    y: u32,
    b: u16,
    ep: u32,
    pc: usize,
    program: String,
    adult: bool,
}
