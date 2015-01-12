// ATTENTION: This file is automatically generated. Don't modify it unless your life is terrible.
#[macro_use]
extern crate data_macro;

// Now, lets get these imports out of the way...
pub use simulation::SimulationState;
pub use graphics::GraphicsState;
pub use core::CoreState;

#[path = "/home/kingsley/Projects/rust/ws/Worldsong/structs/simulation/simulation_state.rs"]
mod simulation;
#[path = "/home/kingsley/Projects/rust/ws/Worldsong/structs/graphics/graphics_state.rs"]
mod graphics;
#[path = "/home/kingsley/Projects/rust/ws/Worldsong/structs/core/core_state.rs"]
mod core;


data! {
    Data {
       simulation: SimulationState = SimulationState::new()
       graphics: GraphicsState = GraphicsState::new()
       core: CoreState = CoreState::new()
    }
}