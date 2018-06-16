use chromosome::Chromosome;
use repo::pg;
use repo::sql;

pub fn init_trade_signals() {
    let conn = pg::connect();
    conn.execute(
        "   DROP TABLE IF EXISTS trade_signals;
            CREATE TABLE trade_signals (
                chromosome_id uuid
            ,   ts date not null
            ,   stratgies text array
            ,   signals integer array
            ,   target_ticker text
            ,   hard_signal integer
            ,   generation integer
            ,   ret numeric
            ,   pnl numeric
            );
            SELECT create_hypertable('trade_signals', 'ts', 'chromosome_id');
            CREATE INDEX ON trade_signals (chromosome_id, ts);",
        &[],
    ).unwrap();
}

pub fn init_chromosomes() {
    let conn = pg::connect();
    conn.execute(
        "   DROP TABLE IF EXISTS trade_chromosomes;
            CREATE TABLE trade_chromosomes (
                id uuid,
                target_ticker text, 
                chromosome text,
                dna integer array,
                generation int,
                chromosome_length int,
                kelly numeric,
                cum_pnl numeric,
                variance numeric,
                mean_return numeric,
                w_kelly numeric,
                num_of_trades integer,
                rank integer
            );",
        &[],
    ).unwrap();
}
