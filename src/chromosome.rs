extern crate uuid;
use uuid::Uuid;

#[derive(Debug)]
pub struct Chromosome {
    pub id: Uuid,
    pub target_ticker: String,
    pub chromosome: String,
    pub dna: Vec<i32>,
    pub generation: i32,
}