
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

pub fn run(worldfile_opt: Option<&String>)
{
    if let Some(worldfile) = worldfile_opt {
        loader::load_and_run(worldfile);
    } else {
        multiverse::run();
    }
}


