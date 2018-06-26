use repo::pg;
use repo::sql;
use forge::Chromosome;

pub fn copy() {
    let conn = pg::connect();
    conn.execute("COPY trade_chromosomes FROM '/tmp/chromosomes.txt';", &[])
        .unwrap();
}

pub fn insert(c: Chromosome) {
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
