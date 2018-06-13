use repo::pg;

pub fn call() {
    let conn = pg::connect();
    conn.execute(" COPY trade_chromosomes FROM '/tmp/chromosomes.txt';", &[])
        .unwrap();
}
