pub mod creature;
pub mod genetics;
pub mod program;
pub mod multiverse;
pub mod utils;
pub mod params;
pub mod plant;
pub mod terrain;
pub mod random;
pub mod log;
pub mod loader;
pub mod world;
pub mod web;

pub fn run(worldfile_opt: Option<String>, run_web: bool)
{
    if let Some(worldfile) = worldfile_opt {
        let mut world = loader::load_world(&worldfile);
        if run_web {
            web::run(world, "resources/ui", "localhost:3000");
        } else {
            world.run();
        }
    } else {
        multiverse::run();
    }
}
