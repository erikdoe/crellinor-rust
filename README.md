# Crellinor

Crellinor is an Artificial Life / Genetic Programming simulation. It places creatures into a world in which they have to survive. The creatures lose one energy point per cycle and they can eat plants to regain a number of energy points. That's the artificial life part. The creatures have individual programs that determine their behaviours. These programs are made up of simple instructions such as `EAT` or `TUL` (for turn left). When two creatures run into each other and certain criteria are met then they mate and produce offspring. The offspring's program is created from the programs of both parents. That's the genetic programming part.

You've probably ended up here because you saw my [Ready for Rust](https://erik.doernenburg.com/talks/#rust) talk and wanted to see more of the code. Well, here it is.

If you actually want to run the simulation you'll notice that there isn't much documentation. Your best bet it is to read the source code and get an idea of how the simulation works. The tests in `tests/integration_tests.rs` could be a good entry point.

Once you get a feel for the simulator you should edit `multiverse.rs`. At the top of the file you can set how many simulations you want to run and how many threads should be used in parallel. Further down in the file is a function named `make_world()` which creates the worlds that should be simulated. The name given to the world is used as a directory name in which the individual simulation results are stored.

You might find the scripts in the `scripts` directory useful to process the results. 
