use repo::pg;

// Initialize trade signals table
pub fn init_trade_signals() {
    info!("Init trade signals table");
    drop_trade_signals();
    create_trade_signals();
    make_trade_signals_hypertable();
    create_index_trade_signals();
}

fn drop_trade_signals() {
    let conn = pg::connect();
    conn.execute("DROP TABLE IF EXISTS trade_signals;", &[])
        .unwrap();
}

fn create_trade_signals() {
    let conn = pg::connect();
    conn.execute(
        "   CREATE TABLE trade_signals (
                chromosome_id uuid
            ,   ts date not null
            ,   stratgies text array
            ,   signals integer array
            ,   target_ticker text
            ,   hard_signal integer
            ,   generation integer
            ,   ret numeric
            ,   pnl numeric
            );",
        &[],
    ).unwrap();
}

fn make_trade_signals_hypertable() {
    let conn = pg::connect();
    conn.execute(
        "SELECT create_hypertable('trade_signals', 'ts', 'chromosome_id');",
        &[],
    ).unwrap();
}

fn create_index_trade_signals() {
    let conn = pg::connect();
    conn.execute("CREATE INDEX ON trade_signals (chromosome_id, ts);", &[])
        .unwrap();
}

// Initialize chromosomes table
pub fn init_chromosomes() {
    info!("Init chromosomes table");
    drop_trade_chromosomes();
    create_trades_chromosomes();
}

fn drop_trade_chromosomes() {
    let conn = pg::connect();
    conn.execute(
        "DROP TABLE IF EXISTS trade_chromosomes;",
        &[],
    ).unwrap();
}

fn create_trades_chromosomes() {
    let conn = pg::connect();
    conn.execute(
        "   CREATE TABLE trade_chromosomes (
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
