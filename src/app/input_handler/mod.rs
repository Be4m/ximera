mod input_handler;
mod input_handler_module;

pub mod modules;

// Reexports
pub use self::{
    input_handler::{InputHandler, InputType, Input},
    input_handler_module::InputHandlerModule,
};