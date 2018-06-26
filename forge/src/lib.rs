//! # Forge where
//!
//! Where does the rest of this go?
//!
extern crate uuid;
extern crate rand;

pub mod chromosome;
pub mod dna;
mod config;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
