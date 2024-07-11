/*
SHARKEY.RS by Alexander Abraham 
a.k.a. "Angel Dollface".
Licensed under the DSL v1.
*/

/// Importing the standard
/// "Result" enum.
use std::fmt::Result;

/// Importing the standard
/// "Display" trait.
use std::fmt::Display;

/// Importing the standard
/// "Error" trait.
use std::error::Error;

/// Importing the standard
/// "Formatter" trait.
use std::fmt::Formatter;

/// A data structure for
/// storing and handling errors.
#[derive(Clone,Eq,PartialEq, Debug)]
pub struct SharkeyErr {
    pub details: String
}

/// Implement generic methods.
impl SharkeyErr {

    /// Implements a generic method to create
    /// a new instance of this data structure.
    pub fn new(details: &str) -> SharkeyErr {
        SharkeyErr {
            details: details.to_owned()
        }
    }
}

/// Implements the "Error" trait.
impl Error for SharkeyErr {
    fn description(&self) -> &str {
        &self.details
    }
}

/// Implements the "Display" trait.
impl Display for SharkeyErr {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f,"{}",self.details)
    }
}
