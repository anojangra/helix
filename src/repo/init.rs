use repo::pg;

// Initialize trade signals table
pub fn init_trade_signals() {
    info!("Init trade signals table");
    let conn = pg::connect();
    conn.batch_execute(
        "   DROP TABLE IF EXISTS trade_signals;
            CREATE TABLE trade_signals (
                chromosome_id uuid
            ,   ts integer not null
            ,   strategies text array
            ,   signals integer array
            ,   target_ticker text
            ,   hard_signal integer
            ,   generation integer
            ,   ret numeric
            ,   pnl numeric
            );
            CREATE INDEX ON trade_signals (chromosome_id, ts);",
    ).unwrap();
}

// Initialize chromosomes table
pub fn init_chromosomes() {
    info!("Init chromosomes table");
    let conn = pg::connect();
    conn.batch_execute(
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
            winning_trades integer,
            losing_trades integer,
            percentage_winners numeric,
            rank integer
            );",
    ).unwrap();
}
