// ATTENTION: This file is automatically generated. How did you even get here? 
// You should probably modify compile_state.rs, the code that generated this file, instead.

// Dependencies
#[macro_use]extern crate data_macro;
#[macro_use]extern crate schedule_macro;
#[macro_use]extern crate sdl2;
#[macro_use]extern crate time;
#[macro_use]extern crate worldsong_config;
#[macro_use]extern crate worldsong_hierarchy;
// State structs
pub use graphics_state::GraphicsState;
pub use core_state::CoreState;
pub use simulation_state::SimulationState;
pub use scheduler_state::SchedulerState;

#[path = "/home/kingsley/WS2/projects/demo/src/state/graphics_state.rs"]
mod graphics_state;
#[path = "/home/kingsley/WS2/projects/demo/src/state/core_state.rs"]
mod core_state;
#[path = "/home/kingsley/WS2/projects/demo/src/state/simulation_state.rs"]
mod simulation_state;
#[path = "/home/kingsley/WS2/projects/demo/src/state/scheduler_state.rs"]
mod scheduler_state;

data! {
    Data {
       graphics_state: GraphicsState = GraphicsState::new()
       core_state: CoreState = CoreState::new()
       simulation_state: SimulationState = SimulationState::new()
       scheduler_state: SchedulerState = SchedulerState::new()
    }
}
