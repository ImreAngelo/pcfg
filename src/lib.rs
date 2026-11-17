//! > **Probablistic Context-Free Grammar (PCFG) password cracker**
//!
//! Description ...

mod grammar;
mod guesser;
mod trainer;

pub use grammar::Grammar;
pub use guesser::Guesses;
pub use trainer::train;
