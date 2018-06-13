use chromosome::Chromosome;
use repo::pg;
use repo::sql;

pub fn call(c: Chromosome) {
    let conn = pg::connect();
    conn.execute(" COPY trade_chromosomes FROM '/tmp/chromosomes.txt';", &[])
        .unwrap();
}
