/*
SHARKEY.RS by Alexander Abraham 
a.k.a. "Angel Dollface".
Licensed under the DSL v1.
*/

/// Exporting the
/// structures to 
/// "help" the main
/// structures.
pub mod aux;

/// Exporting the
/// module that
/// contains functions
/// for getting information
/// on users or notes.
pub mod info;

/// Exporting the 
/// test module.
#[cfg(test)]
pub mod tests;

/// Exporting the 
/// module with needed
/// enums.
pub mod enums;

/// Exporting the 
/// module to handle
/// errors.
pub mod error;

/// Exporting the module
/// containing actions the
/// user needs to be authenticated
/// for.
pub mod actions;

/// Exporting the 
/// module to handle
/// network requests.
pub mod network;

/// Exporting the 
/// module that holds
/// payloads to be submitted.
pub mod payloads;

/// Exporting the module
/// that contains serializable
/// and deserializable responses.
pub mod responses;