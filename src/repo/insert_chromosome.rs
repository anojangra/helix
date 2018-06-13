use chromosome::Chromosome;
use repo::pg;
use repo::sql;

pub fn call(c: Chromosome) {
    let conn = pg::connect();
    conn.execute(
        sql::insert_chromosome::sql(),
        &[
            &c.id,
            &c.target_ticker,
            &c.chromosome,
            &c.dna,
            &c.generation,
            &c.chromosome_length,
        ],
    ).unwrap();
}
