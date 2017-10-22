// Set up all files in the library to use the logging facade.
// Note that configuration is done in the main executable, not in the library.
#[macro_use]
extern crate log;

// Define the modules this library exports.
pub mod input_handlers;
