/*
SHARKEY.RS by Alexander Abraham 
a.k.a. "Angel Dollface".
Licensed under the DSL v1.
*/

/// A structure
/// to bridge the gap
/// between the `Response`
/// structure from `reqwest`
/// module and all kinds of
/// responses.
pub struct Bridge{
    pub body: Option<String>
}