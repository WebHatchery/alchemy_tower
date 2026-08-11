//! Checks over the applied-alchemy layer: that everything a brew can be poured
//! on could actually be treated, and that treating things opens something.
//!
//! Split out of `game_data_progression_tests.rs`, which was at 837 lines when
//! apply targets landed. Those checks ask whether the game can be *finished*;
//! these ask whether its stated premise is on the critical path.

#[cfg(test)]
mod tests;
