/*
SHARKEY.RS by Alexander Abraham 
a.k.a. "Angel Dollface".
Licensed under the DSL v1.
*/

/// Declaring the "modules"
/// directory as a module.
pub mod modules;

/// Re-exporting the 
/// module to handle
/// errors.
pub use modules::error::*;

/// Re-exporting the 
/// module with needed
/// enums.
pub use modules::enums::*;

/// Re-exporting the 
/// module to handle
/// network requests.
pub use modules::network::*;